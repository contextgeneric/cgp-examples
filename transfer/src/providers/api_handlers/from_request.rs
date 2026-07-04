use cgp::prelude::*;

use crate::interfaces::{ApiHandler, ApiHandlerComponent};

#[cgp_impl(new HandleFromRequest<Request, InHandler>)]
#[use_type(HasErrorType.Error)]
#[use_provider(InHandler: ApiHandler<Api>)]
impl<Api, Request, InHandler> ApiHandler<Api>
where
    Request: Into<InHandler::Request>,
{
    type Request = Request;

    type Response = InHandler::Response;

    async fn handle_api(
        &self,
        api: PhantomData<Api>,
        request: Self::Request,
    ) -> Result<Self::Response, Error> {
        InHandler::handle_api(self, api, request.into()).await
    }
}

#[cgp_impl(new HandleFromResponse<Response, InHandler>)]
#[use_type(HasErrorType.Error)]
#[use_provider(InHandler: ApiHandler<Api>)]
impl<Api, Response, InHandler> ApiHandler<Api>
where
    InHandler::Response: Into<Response>,
{
    type Request = InHandler::Request;

    type Response = Response;

    async fn handle_api(
        &self,
        api: PhantomData<Api>,
        request: Self::Request,
    ) -> Result<Self::Response, Error> {
        let response = InHandler::handle_api(self, api, request).await?;

        Ok(response.into())
    }
}
