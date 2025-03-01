use cgp::prelude::*;

use crate::interfaces::*;

#[cgp_auto_getter]
pub trait HasTransferMoneyFields<App>
where
    App: HasUserIdType + HasCurrencyType + HasQuantityType,
{
    fn currency(&self) -> &App::Currency;

    fn recipient(&self) -> &App::UserId;

    fn quantity(&self) -> &App::Quantity;
}

#[cgp_new_provider(ApiHandlerComponent)]
impl<App, Api, Request> ApiHandler<App, Api> for HandleTransfer<Request>
where
    App: CanTransferMoney + CanRaiseHttpError<ErrUnauthorized, String>,
    Request: Async + HasLoggedInUser<App> + HasTransferMoneyFields<App>,
{
    type Request = Request;

    type Response = ();

    async fn handle_api(app: &App, request: Request) -> Result<(), App::Error> {
        let sender = request.logged_in_user().as_ref().ok_or_else(|| {
            App::raise_http_error(
                ErrUnauthorized,
                "you must first login to perform transfer".into(),
            )
        })?;

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
