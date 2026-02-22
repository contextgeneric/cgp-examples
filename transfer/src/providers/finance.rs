use cgp::prelude::*;

use crate::interfaces::*;

#[cgp_impl(new NoTransferToSelf<InHandler>)]
impl<InHandler> MoneyTransferrer
where
    Self: HasUserIdType
        + HasCurrencyType
        + HasQuantityType
        + CanRaiseHttpError<ErrBadRequest, String>,
    InHandler: MoneyTransferrer<Self>,
    Self::UserId: Eq,
{
    async fn transfer_money(
        &self,
        sender: &Self::UserId,
        recipient: &Self::UserId,
        currency: &Self::Currency,
        quantity: &Self::Quantity,
    ) -> Result<(), Self::Error> {
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
