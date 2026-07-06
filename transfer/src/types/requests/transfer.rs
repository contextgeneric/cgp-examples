use axum::extract::Query;
use axum_extra::TypedHeader;
use cgp::prelude::*;
use headers::Authorization;
use headers::authorization::Basic;
use serde::Deserialize;

use crate::types::DemoCurrency;

/// The domain request the transfer handler works with. `#[derive(HasField)]` exposes each
/// field by name so the `HasTransferMoneyFields`/`HasLoggedInUser`/`HasBasicAuthHeader`
/// getters can read it generically.
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

/// The query-string fields Axum deserializes for a transfer request.
#[derive(Deserialize)]
pub struct TransferQuery {
    pub currency: DemoCurrency,
    pub recipient: String,
    pub quantity: u64,
}

/// The raw type Axum extracts from the wire; `HandleFromRequest` converts it into
/// `TransferRequest` so the endpoint handler stays free of HTTP extractor types.
pub type AxumTransferRequest = (
    Query<TransferQuery>,
    Option<TypedHeader<Authorization<Basic>>>,
);
