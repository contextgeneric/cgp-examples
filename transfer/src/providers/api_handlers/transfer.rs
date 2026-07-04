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

#[cgp_impl(new HandleTransfer<Request>)]
#[uses(CanTransferMoney, CanRaiseHttpError<ErrUnauthorized, String>)]
#[use_type(HasErrorType.Error)]
impl<Api, Request> ApiHandler<Api>
where
    Request: HasLoggedInUser<Self> + HasTransferMoneyFields<Self>,
{
    type Request = Request;

    type Response = ();

    async fn handle_api(
        &self,
        _api: PhantomData<Api>,
        request: Request,
    ) -> Result<(), Error> {
        let sender = request.logged_in_user().as_ref().ok_or_else(|| {
            Self::raise_http_error(
                ErrUnauthorized,
                "you must first login to perform transfer".into(),
            )
        })?;

        self.transfer_money(
            sender,
            request.recipient(),
            request.currency(),
            request.quantity(),
        )
        .await?;

        Ok(())
    }
}
