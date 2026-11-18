use axum::extract::Query;
use axum_extra::TypedHeader;
use cgp::prelude::*;
use headers::Authorization;
use headers::authorization::Basic;
use serde::Deserialize;

use crate::types::DemoCurrency;

/// The domain request the balance handler works with. `#[derive(HasField)]` exposes each
/// field by name, which is what lets the `HasQueryBalanceFields`/`HasLoggedInUser` getters
/// (and the auth wrapper's `HasBasicAuthHeader`) read it generically.
#[derive(HasField)]
pub struct QueryBalanceRequest {
    pub currency: DemoCurrency,
    pub basic_auth_header: Option<(String, String)>,
    pub logged_in_user: Option<String>,
}

impl From<AxumQueryBalanceRequest> for QueryBalanceRequest {
    fn from((Query(query), auth): AxumQueryBalanceRequest) -> Self {
        let basic_auth_header = auth.map(|TypedHeader(Authorization(basic))| {
            (basic.username().into(), basic.password().into())
        });

        Self {
            currency: query.currency,
            basic_auth_header,
            logged_in_user: None,
        }
    }
}

/// The raw type Axum extracts from the wire (query string plus optional auth header).
/// `HandleFromRequest` converts it into `QueryBalanceRequest` via the `From` impl above, so
/// the endpoint handler never sees the HTTP-specific extractor types.
pub type AxumQueryBalanceRequest = (
    Query<QueryBalanceQuery>,
    Option<TypedHeader<Authorization<Basic>>>,
);

/// The query-string fields Axum deserializes for a balance request.
#[derive(Deserialize)]
pub struct QueryBalanceQuery {
    pub currency: DemoCurrency,
}
