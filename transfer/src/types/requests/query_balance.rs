use axum::extract::Query;
use axum_extra::TypedHeader;
use cgp::prelude::*;
use headers::authorization::Basic;
use headers::Authorization;
use serde::Deserialize;

use crate::types::DemoCurrency;

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

pub type AxumQueryBalanceRequest = (
    Query<QueryBalanceQuery>,
    Option<TypedHeader<Authorization<Basic>>>,
);

#[derive(Deserialize)]
pub struct QueryBalanceQuery {
    pub currency: DemoCurrency,
}
