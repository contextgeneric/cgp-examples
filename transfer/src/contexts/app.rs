use std::collections::BTreeMap;
use std::sync::Arc;

use cgp::core::component::UseDelegate;
use cgp::core::error::ErrorTypeComponent;
use cgp::prelude::*;
use futures::lock::Mutex;

use crate::interfaces::*;
use crate::providers::*;
use crate::types::{AppError, DemoCurrency};

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
            UserBalanceQuerierComponent,
            MoneyTransferrerComponent,
        ]:
            UseMockedApp,
    }
}

pub struct HandleAppErrors;

delegate_components! {
    HandleAppErrors {
        <Code> (Code, String): DisplayHttpError,
        <Code> (Code, anyhow::Error): HandleHttpErrorWithAnyhow,
    }
}

pub trait CanUseMockedAppComponents:
    CanUseComponent<UserBalanceQuerierComponent> + CanUseComponent<MoneyTransferrerComponent>
{
}

impl CanUseMockedAppComponents for MockApp {}
