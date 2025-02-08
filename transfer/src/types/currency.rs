use core::fmt::Display;

use serde::Deserialize;

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
