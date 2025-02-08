use core::marker::PhantomData;

use cgp::core::component::UseDelegate;
use cgp::prelude::*;
use serde::Serialize;

use crate::interfaces::*;

#[cgp_provider(ApiHandlerComponent)]
impl<App, Api, Components, Delegate> ApiHandler<App, Api> for UseDelegate<Components>
where
    App: HasAsyncErrorType,
    Components: DelegateComponent<Api, Delegate = Delegate>,
    Delegate: ApiHandler<App, Api>,
{
    type Request = Delegate::Request;

    type Response = Delegate::Response;

    async fn handle_api(
        app: &mut App,
        request: Self::Request,
    ) -> Result<Self::Response, App::Error> {
        Delegate::handle_api(app, request).await
    }
}

pub struct HandleTransfer<Request>(pub PhantomData<Request>);

#[cgp_auto_getter]
pub trait HasTransferMoneyFields<App>
where
    App: HasUserIdType + HasCurrencyType + HasQuantityType,
{
    fn currency(&self) -> &App::Currency;

    fn recipient(&self) -> &App::UserId;

    fn quantity(&self) -> &App::Quantity;
}

impl<App, Api, Request> ApiHandler<App, Api> for HandleTransfer<Request>
where
    App: CanTransferMoney,
    Request: Async + HasTransferMoneyFields<App>,
{
    type Request = Request;

    type Response = ();

    async fn handle_api(app: &mut App, request: Request) -> Result<(), App::Error> {
        app.transfer_money(request.recipient(), request.currency(), request.quantity())
            .await?;

        Ok(())
    }
}

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

impl<App, Api, Request> ApiHandler<App, Api> for HandleQueryBalance<Request>
where
    App: CanQueryCurrentUserBalance,
    Request: Async + HasQueryBalanceFields<App>,
{
    type Request = Request;

    type Response = QueryBalanceResponse<App>;

    async fn handle_api(
        app: &mut App,
        request: Request,
    ) -> Result<QueryBalanceResponse<App>, App::Error> {
        let balance = app.query_current_user_balance(request.currency()).await?;

        Ok(QueryBalanceResponse { balance })
    }
}
