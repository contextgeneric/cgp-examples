use axum::extract::Query;
use cgp::prelude::*;
use headers::authorization::Basic;
use headers::Authorization;
use serde::Deserialize;

use crate::types::DemoCurrency;

#[derive(HasField)]
pub struct QueryBalanceRequest {
    pub currency: DemoCurrency,
    pub auth_header: Option<(String, String)>,
    pub logged_in_user: Option<String>,
}

impl From<AxumQueryBalanceRequest> for QueryBalanceRequest {
    fn from((Query(query), auth): AxumQueryBalanceRequest) -> Self {
        Self {
            currency: query.currency,
            auth_header: auth
                .map(|Authorization(basic)| (basic.username().into(), basic.password().into())),
            logged_in_user: None,
        }
    }
}

pub type AxumQueryBalanceRequest = (Query<QueryBalanceQuery>, Option<Authorization<Basic>>);

#[derive(Deserialize)]
pub struct QueryBalanceQuery {
    pub currency: DemoCurrency,
}
