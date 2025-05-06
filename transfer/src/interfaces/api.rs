use cgp::core::component::UseDelegate;
use cgp::prelude::*;

#[cgp_component {
    provider: ApiHandler,
    context: App,
}]
#[async_trait]
pub trait CanHandleApi<Api>: HasAsyncErrorType {
    type Request: Send + Sync;

    type Response: Send + Sync;

    async fn handle_api(&self, request: Self::Request) -> Result<Self::Response, Self::Error>;
}

pub struct TransferApi;

pub struct QueryBalanceApi;

#[cgp_provider(ApiHandlerComponent)]
impl<App, Api, Components, Delegate> ApiHandler<App, Api> for UseDelegate<Components>
where
    App: HasAsyncErrorType,
    Components: DelegateComponent<Api, Delegate = Delegate>,
    Delegate: ApiHandler<App, Api>,
{
    type Request = Delegate::Request;

    type Response = Delegate::Response;

    async fn handle_api(app: &App, request: Self::Request) -> Result<Self::Response, App::Error> {
        Delegate::handle_api(app, request).await
    }
}
