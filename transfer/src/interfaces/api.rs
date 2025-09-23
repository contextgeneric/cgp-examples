use cgp::core::component::UseDelegate;
use cgp::prelude::*;

#[cgp_component{
    provider: ApiHandler,
    derive_delegate: UseDelegate<Api>,
}]
#[async_trait]
pub trait CanHandleApi<Api>: HasErrorType {
    type Request;

    type Response;

    async fn handle_api(
        &self,
        _api: PhantomData<Api>,
        request: Self::Request,
    ) -> Result<Self::Response, Self::Error>;
}

pub trait CanHandleApiSend<Api>:
    CanHandleApi<Api, Request: Send, Response: Send> + Send + Sync
{
    fn handle_api_send(
        &self,
        _api: PhantomData<Api>,
        request: Self::Request,
    ) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send;
}

pub struct TransferApi;

pub struct QueryBalanceApi;
