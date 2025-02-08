use axum::extract::Query;
use cgp::prelude::*;
use headers::authorization::Basic;
use headers::Authorization;
use serde::Deserialize;

use crate::types::DemoCurrency;

#[derive(HasField)]
pub struct TransferRequest {
    pub currency: DemoCurrency,
    pub recipient: String,
    pub quantity: u64,
    pub auth_header: Option<(String, String)>,
}

impl From<AxumTransferRequest> for TransferRequest {
    fn from((Query(query), auth): AxumTransferRequest) -> Self {
        Self {
            currency: query.currency,
            recipient: query.recipient,
            quantity: query.quantity,
            auth_header: auth
                .map(|Authorization(basic)| (basic.username().into(), basic.password().into())),
        }
    }
}

#[derive(Deserialize)]
pub struct TransferQuery {
    pub currency: DemoCurrency,
    pub recipient: String,
    pub quantity: u64,
}

pub type AxumTransferRequest = (Query<TransferQuery>, Option<Authorization<Basic>>);
