use axum::extract::Query;
use axum_extra::TypedHeader;
use cgp::prelude::*;
use headers::Authorization;
use headers::authorization::Basic;
use serde::Deserialize;

use crate::types::DemoCurrency;

#[derive(HasField)]
pub struct TransferRequest {
    pub currency: DemoCurrency,
    pub recipient: String,
    pub quantity: u64,
    pub basic_auth_header: Option<(String, String)>,
    pub logged_in_user: Option<String>,
}

impl From<AxumTransferRequest> for TransferRequest {
    fn from((Query(query), auth): AxumTransferRequest) -> Self {
        let basic_auth_header = auth.map(|TypedHeader(Authorization(basic))| {
            (basic.username().into(), basic.password().into())
        });

        Self {
            currency: query.currency,
            recipient: query.recipient,
            quantity: query.quantity,
            basic_auth_header,
            logged_in_user: None,
        }
    }
}

#[derive(Deserialize)]
pub struct TransferQuery {
    pub currency: DemoCurrency,
    pub recipient: String,
    pub quantity: u64,
}

pub type AxumTransferRequest = (
    Query<TransferQuery>,
    Option<TypedHeader<Authorization<Basic>>>,
);
