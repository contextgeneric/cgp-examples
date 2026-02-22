use cgp::prelude::*;
use serde::Serialize;

use crate::interfaces::*;

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

#[cgp_impl(new HandleQueryBalance<Request>)]
impl<Api, Request> ApiHandler<Api>
where
    Self: CanQueryUserBalance + CanRaiseHttpError<ErrUnauthorized, String>,
    Request: HasLoggedInUser<Self> + HasQueryBalanceFields<Self>,
{
    type Request = Request;

    type Response = QueryBalanceResponse<Self>;

    async fn handle_api(
        &self,
        _api: PhantomData<Api>,
        request: Request,
    ) -> Result<QueryBalanceResponse<Self>, Self::Error> {
        let user = request.logged_in_user().as_ref().ok_or_else(|| {
            Self::raise_http_error(ErrUnauthorized, "you must first login".into())
        })?;

        let balance = self.query_user_balance(user, request.currency()).await?;

        Ok(QueryBalanceResponse { balance })
    }
}
