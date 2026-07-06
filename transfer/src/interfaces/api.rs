use cgp::prelude::*;

#[cgp_component(ApiHandler)]
#[prefix(@app.api in DefaultNamespace)]
#[async_trait]
#[use_type(HasErrorType.Error)]
pub trait CanHandleApi<Api> {
    type Request;

    type Response;

    async fn handle_api(
        &self,
        _api: PhantomData<Api>,
        request: Self::Request,
    ) -> Result<Self::Response, Error>;
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
