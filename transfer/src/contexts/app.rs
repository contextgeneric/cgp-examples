use std::collections::BTreeMap;
use std::sync::Arc;

use axum::Router;
use cgp::core::component::UseDelegate;
use cgp::core::error::ErrorTypeComponent;
use cgp::prelude::*;
use futures::lock::Mutex;

use crate::interfaces::*;
use crate::providers::*;
use crate::types::{
    AppError, AxumQueryBalanceRequest, AxumTransferRequest, DemoCurrency, QueryBalanceRequest,
    TransferRequest,
};

#[derive(HasField)]
pub struct MockApp {
    pub user_balances: Arc<Mutex<BTreeMap<(String, DemoCurrency), u64>>>,
    pub user_passwords: BTreeMap<String, String>,
}

pub struct MockAppComponents;

impl HasComponents for MockApp {
    type Components = MockAppComponents;
}

delegate_components! {
    MockAppComponents {
        ErrorTypeComponent: UseType<AppError>,
        HttpErrorRaiserComponent: UseDelegate<HandleAppErrors>,
        [
            UserIdTypeComponent,
            PasswordTypeComponent,
            HashedPasswordTypeComponent,
        ]:
            UseType<String>,
        QuantityTypeComponent:
            UseType<u64>,
        CurrencyTypeComponent:
            UseType<DemoCurrency>,
        [
            PasswordCheckerComponent,
            UserHashedPasswordQuerierComponent,
            UserBalanceQuerierComponent,
            MoneyTransferrerComponent,
        ]:
            UseMockedApp,
        ApiHandlerComponent: UseDelegate<ApiHandlers>,
    }
}

pub struct HandleAppErrors;

delegate_components! {
    HandleAppErrors {
        <Code> (Code, String): DisplayHttpError,
        <Code> (Code, anyhow::Error): HandleHttpErrorWithAnyhow,
    }
}

pub struct ApiHandlers;

delegate_components! {
    ApiHandlers {
        QueryBalanceApi:
            HandleFromRequest<
                AxumQueryBalanceRequest,
                ResponseToJson<
                    UseBasicAuth<
                        HandleQueryBalance<QueryBalanceRequest>
                    >>>,
        TransferApi:
            HandleFromRequest<
                AxumTransferRequest,
                UseBasicAuth<
                    HandleTransfer<TransferRequest>
                >
            >,
    }
}

pub trait CanUseMockAppComponents:
    CanUseComponent<UserBalanceQuerierComponent>
    + CanUseComponent<MoneyTransferrerComponent>
    + CanUseComponent<ApiHandlerComponent, QueryBalanceApi>
    + CanUseComponent<ApiHandlerComponent, TransferApi>
{
}

impl CanUseMockAppComponents for MockApp {}

pub trait CanAddApiRoutes:
    CanAddRoute<MockApp, QueryBalanceApi, GetMethod> + CanAddRoute<MockApp, TransferApi, PostMethod>
{
}

impl CanAddApiRoutes for Router<Arc<MockApp>> {}
