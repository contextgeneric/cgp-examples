use serde::Deserialize;

#[derive(Deserialize)]
pub enum DemoCurrency {
    EUR,
    USD,
}
