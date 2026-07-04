use cgp::prelude::*;

use crate::interfaces::*;

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
