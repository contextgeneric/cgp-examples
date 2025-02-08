use cgp::prelude::*;

#[cgp_component {
    provider: ApiHandler,
}]
#[async_trait]
pub trait CanHandleApi<Api>: HasAsyncErrorType {
    type Request: Async;

    type Response: Async;

    async fn handle_api(&mut self, request: Self::Request) -> Result<Self::Response, Self::Error>;
}
