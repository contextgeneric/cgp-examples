use cgp::prelude::*;

use crate::interfaces::{ApiHandler, ApiHandlerComponent};

// A higher-order `ApiHandler` provider that adapts the request type: it accepts an outer
// `Request` (e.g. the raw type the HTTP layer produces), converts it into the inner handler's
// request via `Into`, and delegates. This lets an endpoint be written against a clean domain
// request while sitting behind the raw extractor type.
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

// The mirror wrapper that adapts the response type: it runs the inner handler and converts
// its response into the outer `Response` via `Into`. (Defined for completeness alongside
// `HandleFromRequest`.)
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
