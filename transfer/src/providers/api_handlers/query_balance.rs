use cgp::prelude::*;
use serde::Serialize;

use crate::interfaces::*;

// Getter for the fields a balance query needs off its request struct.
#[cgp_auto_getter]
pub trait HasQueryBalanceFields<App>
where
    App: HasCurrencyType,
{
    fn currency(&self) -> &App::Currency;
}

/// The balance-query response, generic over the context's abstract `Quantity` so it stays
/// backend-independent; `Serialize` lets the JSON wrapper encode it.
#[derive(Serialize)]
pub struct QueryBalanceResponse<App>
where
    App: HasQuantityType,
{
    pub balance: App::Quantity,
}

// The balance endpoint: an `ApiHandler` provider that requires the caller to be logged in,
// then queries the balance. It depends only on the `CanQueryUserBalance` capability and
// error-raising (declared with `#[uses(...)]`), not on any concrete store, and works for any
// request exposing the fields named in its `where` clause.
#[cgp_impl(new HandleQueryBalance<Request>)]
#[uses(CanQueryUserBalance, CanRaiseHttpError<ErrUnauthorized, String>)]
#[use_type(HasErrorType.Error)]
impl<Api, Request> ApiHandler<Api>
where
    Request: HasLoggedInUser<Self> + HasQueryBalanceFields<Self>,
{
    type Request = Request;

    type Response = QueryBalanceResponse<Self>;

    async fn handle_api(
        &self,
        _api: PhantomData<Api>,
        request: Request,
    ) -> Result<QueryBalanceResponse<Self>, Error> {
        let user = request.logged_in_user().as_ref().ok_or_else(|| {
            Self::raise_http_error(ErrUnauthorized, "you must first login".into())
        })?;

        let balance = self.query_user_balance(user, request.currency()).await?;

        Ok(QueryBalanceResponse { balance })
    }
}
