use cgp::prelude::*;

use crate::interfaces::{ApiHandler, ApiHandlerComponent};

#[cgp_new_provider]
impl<App, Api, Request, InHandler> ApiHandler<App, Api> for HandleFromRequest<Request, InHandler>
where
    App: HasErrorType,
    InHandler: ApiHandler<App, Api>,
    Request: Into<InHandler::Request>,
{
    type Request = Request;

    type Response = InHandler::Response;

    async fn handle_api(
        app: &App,
        api: PhantomData<Api>,
        request: Self::Request,
    ) -> Result<Self::Response, <App>::Error> {
        InHandler::handle_api(app, api, request.into()).await
    }
}

#[cgp_new_provider]
impl<App, Api, Response, InHandler> ApiHandler<App, Api> for HandleFromResponse<Response, InHandler>
where
    App: HasErrorType,
    InHandler: ApiHandler<App, Api>,
    InHandler::Response: Into<Response>,
{
    type Request = InHandler::Request;

    type Response = Response;

    async fn handle_api(
        app: &App,
        api: PhantomData<Api>,
        request: Self::Request,
    ) -> Result<Self::Response, <App>::Error> {
        let response = InHandler::handle_api(app, api, request).await?;

        Ok(response.into())
    }
}
