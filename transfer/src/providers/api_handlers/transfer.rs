use core::marker::PhantomData;

use cgp::prelude::*;

use crate::interfaces::*;

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

#[cgp_provider(ApiHandlerComponent)]
impl<App, Api, Request> ApiHandler<App, Api> for HandleTransfer<Request>
where
    App: CanTransferMoney + CanRaiseAsyncError<String>,
    Request: Async + HasLoggedInUser<App> + HasTransferMoneyFields<App>,
{
    type Request = Request;

    type Response = ();

    async fn handle_api(app: &App, request: Request) -> Result<(), App::Error> {
        let sender = request
            .logged_in_user()
            .as_ref()
            .ok_or_else(|| App::raise_error("you must first login to perform transfer".into()))?;

        app.transfer_money(
            &sender,
            request.recipient(),
            request.currency(),
            request.quantity(),
        )
        .await?;

        Ok(())
    }
}
