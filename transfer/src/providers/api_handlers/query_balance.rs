use core::marker::PhantomData;

use cgp::prelude::*;
use serde::Serialize;

use crate::interfaces::*;

pub struct HandleQueryBalance<Request>(pub PhantomData<Request>);

#[cgp_auto_getter]
pub trait HasQueryBalanceFields<App>
where
    App: HasCurrencyType,
{
    fn currency(&self) -> &App::Currency;
}

#[derive(Serialize)]
pub struct QueryBalanceResponse<App>
where
    App: HasQuantityType,
{
    pub balance: App::Quantity,
}

#[cgp_provider(ApiHandlerComponent)]
impl<App, Api, Request> ApiHandler<App, Api> for HandleQueryBalance<Request>
where
    App: CanQueryUserBalance + CanRaiseHttpError<ErrUnauthorized, String>,
    Request: Async + HasLoggedInUser<App> + HasQueryBalanceFields<App>,
{
    type Request = Request;

    type Response = QueryBalanceResponse<App>;

    async fn handle_api(
        app: &App,
        request: Request,
    ) -> Result<QueryBalanceResponse<App>, App::Error> {
        let user = request
            .logged_in_user()
            .as_ref()
            .ok_or_else(|| App::raise_http_error(ErrUnauthorized, "you must first login".into()))?;

        let balance = app.query_user_balance(user, request.currency()).await?;

        Ok(QueryBalanceResponse { balance })
    }
}
