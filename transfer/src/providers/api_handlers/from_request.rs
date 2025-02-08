use core::marker::PhantomData;

use cgp::prelude::*;

use crate::interfaces::{ApiHandler, ApiHandlerComponent};

pub struct HandleFromRequest<Request, InHandler>(pub PhantomData<(Request, InHandler)>);

#[cgp_provider(ApiHandlerComponent)]
impl<App, Api, Request, InHandler> ApiHandler<App, Api> for HandleFromRequest<Request, InHandler>
where
    App: HasAsyncErrorType,
    InHandler: ApiHandler<App, Api>,
    InHandler::Request: From<Request>,
    Request: Async,
{
    type Request = Request;

    type Response = InHandler::Response;

    async fn handle_api(
        app: &mut App,
        request: Self::Request,
    ) -> Result<Self::Response, <App>::Error> {
        InHandler::handle_api(app, request.into()).await
    }
}

pub struct HandleFromResponse<Response, InHandler>(pub PhantomData<(Response, InHandler)>);

#[cgp_provider(ApiHandlerComponent)]
impl<App, Api, Response, InHandler> ApiHandler<App, Api> for HandleFromResponse<Response, InHandler>
where
    App: HasAsyncErrorType,
    InHandler: ApiHandler<App, Api>,
    Response: Async + From<InHandler::Response>,
{
    type Request = InHandler::Request;

    type Response = Response;

    async fn handle_api(
        app: &mut App,
        request: Self::Request,
    ) -> Result<Self::Response, <App>::Error> {
        let response = InHandler::handle_api(app, request).await?;

        Ok(response.into())
    }
}
