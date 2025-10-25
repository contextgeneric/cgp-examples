use cgp::prelude::*;

use crate::interfaces::{ApiHandler, ApiHandlerComponent};

#[cgp_impl(new HandleFromRequest<Request, InHandler>)]
impl<App, Api, Request, InHandler> ApiHandler<Api> for App
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

#[cgp_impl(new HandleFromResponse<Response, InHandler>)]
impl<App, Api, Response, InHandler> ApiHandler<Api> for App
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
