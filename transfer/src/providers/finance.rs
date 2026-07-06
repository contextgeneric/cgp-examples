use cgp::prelude::*;

use crate::interfaces::*;

// A higher-order provider for `MoneyTransferrer`: it wraps an inner transfer provider,
// rejecting a self-transfer up front and otherwise delegating. `#[use_provider(InHandler:
// MoneyTransferrer)]` declares the inner-provider dependency and fills in the hidden context
// argument, so the body calls it as the associated function `InHandler::transfer_money(self, ...)`.
#[cgp_impl(new NoTransferToSelf<InHandler>)]
#[use_type(
    HasUserIdType.UserId,
    HasCurrencyType.Currency,
    HasQuantityType.Quantity,
    HasErrorType.Error,
)]
#[uses(CanRaiseHttpError<ErrBadRequest, String>)]
#[use_provider(InHandler: MoneyTransferrer)]
impl<InHandler> MoneyTransferrer
where
    UserId: Eq,
{
    async fn transfer_money(
        &self,
        sender: &UserId,
        recipient: &UserId,
        currency: &Currency,
        quantity: &Quantity,
    ) -> Result<(), Error> {
        if sender != recipient {
            InHandler::transfer_money(self, sender, recipient, currency, quantity).await
        } else {
            Err(Self::raise_http_error(
                ErrBadRequest,
                format!("cannot transfer with the same sender and recipient: {sender}"),
            ))
        }
    }
}
