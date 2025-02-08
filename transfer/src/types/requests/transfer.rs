use cgp::prelude::*;

use crate::types::DemoCurrency;

#[derive(HasField)]
pub struct TransferRequest {
    pub currency: DemoCurrency,
    pub recipient: String,
    pub quantity: u64,
    pub auth_header: Option<(String, String)>,
}
