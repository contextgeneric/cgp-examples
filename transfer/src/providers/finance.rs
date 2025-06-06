use cgp::prelude::*;

use crate::interfaces::*;

#[cgp_new_provider]
impl<App, InHandler> MoneyTransferrer<App> for NoTransferToSelf<InHandler>
where
    App: HasUserIdType
        + HasCurrencyType
        + HasQuantityType
        + CanRaiseHttpError<ErrBadRequest, String>,
    InHandler: MoneyTransferrer<App>,
    App::UserId: Eq,
{
    async fn transfer_money(
        app: &App,
        sender: &App::UserId,
        recipient: &App::UserId,
        currency: &App::Currency,
        quantity: &App::Quantity,
    ) -> Result<(), App::Error> {
        if sender != recipient {
            InHandler::transfer_money(app, sender, recipient, currency, quantity).await
        } else {
            Err(App::raise_http_error(
                ErrBadRequest,
                format!("cannot transfer with the same sender and recipient: {sender}"),
            ))
        }
    }
}
