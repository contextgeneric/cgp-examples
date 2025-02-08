use cgp::core::component::UseDelegate;
use cgp::prelude::*;

#[cgp_component {
    provider: ApiHandler,
    context: App,
}]
#[async_trait]
pub trait CanHandleApi<Api>: HasAsyncErrorType {
    type Request: Async;

    type Response: Async;

    async fn handle_api(&mut self, request: Self::Request) -> Result<Self::Response, Self::Error>;
}

#[cgp_provider(ApiHandlerComponent)]
impl<App, Api, Components, Delegate> ApiHandler<App, Api> for UseDelegate<Components>
where
    App: HasAsyncErrorType,
    Components: DelegateComponent<Api, Delegate = Delegate>,
    Delegate: ApiHandler<App, Api>,
{
    type Request = Delegate::Request;

    type Response = Delegate::Response;

    async fn handle_api(
        app: &mut App,
        request: Self::Request,
    ) -> Result<Self::Response, App::Error> {
        Delegate::handle_api(app, request).await
    }
}
