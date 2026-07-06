use std::collections::BTreeMap;
use std::sync::Arc;

use axum::Router;
use cgp::prelude::*;
use futures::lock::Mutex;

use crate::interfaces::*;
use crate::namespaces::{DefaultApiHandlers, MockNamespace};
use crate::providers::*;
use crate::types::DemoCurrency;

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

pub trait CanAddApiRoutes: CanAddMainApiRoutes<MockApp> {}

impl CanAddApiRoutes for Router<Arc<MockApp>> {}
