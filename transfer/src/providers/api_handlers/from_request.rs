use cgp::prelude::*;

use crate::interfaces::{ApiHandler, ApiHandlerComponent};

#[new_cgp_provider(ApiHandlerComponent)]
impl<App, Api, Request, InHandler> ApiHandler<App, Api> for HandleFromRequest<Request, InHandler>
where
    App: HasAsyncErrorType,
    InHandler: ApiHandler<App, Api>,
    Request: Into<InHandler::Request>,
    Request: Async,
{
    type Request = Request;

    type Response = InHandler::Response;

    async fn handle_api(app: &App, request: Self::Request) -> Result<Self::Response, <App>::Error> {
        InHandler::handle_api(app, request.into()).await
    }
}

#[new_cgp_provider(ApiHandlerComponent)]
impl<App, Api, Response, InHandler> ApiHandler<App, Api> for HandleFromResponse<Response, InHandler>
where
    App: HasAsyncErrorType,
    InHandler: ApiHandler<App, Api>,
    InHandler::Response: Into<Response>,
    Response: Async,
{
    type Request = InHandler::Request;

    type Response = Response;

    async fn handle_api(app: &App, request: Self::Request) -> Result<Self::Response, <App>::Error> {
        let response = InHandler::handle_api(app, request).await?;

        Ok(response.into())
    }
}
