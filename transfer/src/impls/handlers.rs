use core::marker::PhantomData;

use cgp::core::component::UseDelegate;
use cgp::prelude::*;

use crate::traits::*;

pub struct AuthenticateUser<InHandler>(pub PhantomData<InHandler>);

#[cgp_provider(ApiHandlerComponent)]
impl<App, Api, InHandler> ApiHandler<App, Api> for AuthenticateUser<InHandler>
where
    App: CanAuthenticateUser,
    InHandler: ApiHandler<App, Api>,
{
    type Request = InHandler::Request;

    type Response = InHandler::Response;

    async fn handle_api(
        app: &mut App,
        request: &Self::Request,
    ) -> Result<Self::Response, App::Error> {
        app.authenticate_user().await?;

        InHandler::handle_api(app, request).await
    }
}

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
        request: &Self::Request,
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

    async fn handle_api(app: &mut App, request: &Request) -> Result<(), App::Error> {
        app.transfer_money(request.recipient(), request.currency(), request.quantity())
            .await?;

        Ok(())
    }
}
