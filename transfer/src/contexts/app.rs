use std::collections::BTreeMap;
use std::sync::Arc;

use axum::Router;
use cgp::prelude::*;
use futures::lock::Mutex;

use crate::interfaces::*;
use crate::namespaces::{DefaultApiHandlers, MockNamespace};
use crate::providers::*;
use crate::types::DemoCurrency;

/// The concrete application context — the type all the abstract capabilities are wired onto.
/// Its fields hold the in-memory data the mock backend reads; `#[derive(HasField)]` exposes
/// them by name so `UseMockedApp`'s `#[implicit]` arguments can pull them in.
#[derive(HasField, Default)]
pub struct MockApp {
    pub user_balances: Arc<Mutex<BTreeMap<(String, DemoCurrency), u64>>>,
    pub user_passwords: BTreeMap<String, String>,
}

impl MockApp {
    pub fn new_with_dummy_data() -> Self {
        let user_balances = BTreeMap::from([
            (("alice".into(), DemoCurrency::EUR), 100),
            (("alice".into(), DemoCurrency::USD), 50),
            (("bob".into(), DemoCurrency::EUR), 200),
            (("bob".into(), DemoCurrency::USD), 150),
        ]);

        let user_passwords = BTreeMap::from([
            ("alice".into(), "wonderland".into()),
            ("bob".into(), "sponge".into()),
        ]);

        Self {
            user_balances: Arc::new(Mutex::new(user_balances)),
            user_passwords,
        }
    }
}

// The whole application specified as a wiring table. Three decisions: join `MockNamespace`
// to inherit every default backend and abstract type; pull each `DefaultApiHandlers` entry
// onto the `ApiHandler` dispatch path (one entry per endpoint) with the `for` loop; and
// override the money-transfer path to wrap the mock backend in `NoTransferToSelf`. The
// override is legal only because `MockNamespace` does not itself claim that path.
delegate_components! {
    MockApp {
        namespace MockNamespace;

        for <Key, Value> in DefaultApiHandlers {
            @app.api.ApiHandlerComponent.Key: Value,
        }

        @app.finance.MoneyTransferrerComponent:
            NoTransferToSelf<UseMockedApp>,
    }
}

// Compile-time proof that the wiring above is complete: because CGP resolves wiring lazily,
// this block forces the compiler to check each listed component (and each API marker for the
// generic `ApiHandler`) is fully satisfied, reporting any missing dependency at this site.
check_components! {
    MockApp
    {
        QuantityTypeProviderComponent,
        UserBalanceQuerierComponent,
        MoneyTransferrerComponent,
        ApiHandlerComponent: [
            QueryBalanceApi,
            TransferApi,
        ],
    }
}

// Recover the `Send` bound Axum needs. `CanHandleApiSend` cannot be implemented generically
// on stable Rust, so it is implemented per concrete (context, API) pair: at a fixed type the
// awaited future is concrete and the compiler can confirm it is `Send`. Each impl just
// forwards to `handle_api`.
impl CanHandleApiSend<QueryBalanceApi> for MockApp {
    async fn handle_api_send(
        &self,
        api: PhantomData<QueryBalanceApi>,
        request: Self::Request,
    ) -> Result<Self::Response, Self::Error> {
        self.handle_api(api, request).await
    }
}

impl CanHandleApiSend<TransferApi> for MockApp {
    async fn handle_api_send(
        &self,
        api: PhantomData<TransferApi>,
        request: Self::Request,
    ) -> Result<Self::Response, Self::Error> {
        self.handle_api(api, request).await
    }
}

/// Convenience alias binding the generic routing capability to `MockApp`, so `bin/server.rs`
/// can mount the service on a `Router<Arc<MockApp>>`.
pub trait CanAddApiRoutes: CanAddMainApiRoutes<MockApp> {}

impl CanAddApiRoutes for Router<Arc<MockApp>> {}
