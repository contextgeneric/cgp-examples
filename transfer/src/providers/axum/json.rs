use axum::Json;
use cgp::prelude::*;

use crate::interfaces::{ApiHandler, ApiHandlerComponent};

#[cgp_impl(new ResponseToJson<InHandler>)]
#[use_type(HasErrorType.Error)]
#[use_provider(InHandler: ApiHandler<Api>)]
impl<Api, InHandler> ApiHandler<Api> {
    type Request = InHandler::Request;

    type Response = Json<InHandler::Response>;

    async fn handle_api(
        &self,
        api: PhantomData<Api>,
        request: Self::Request,
    ) -> Result<Self::Response, Error> {
        let response = InHandler::handle_api(self, api, request).await?;
        Ok(Json(response))
    }
}
