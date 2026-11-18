use core::fmt::Display;

use serde::Deserialize;

/// The concrete currency this deployment uses. `MockApp` wires it as the app's abstract
/// `Currency` type (`CurrencyTypeProviderComponent: UseType<DemoCurrency>`); another
/// deployment could plug in a different type without changing any handler.
#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Deserialize)]
pub enum DemoCurrency {
    EUR,
    USD,
}

impl Display for DemoCurrency {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DemoCurrency::EUR => write!(f, "EUR"),
            DemoCurrency::USD => write!(f, "USD"),
        }
    }
}
