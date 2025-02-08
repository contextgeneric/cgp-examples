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
