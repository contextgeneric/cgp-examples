use core::marker::PhantomData;

use axum::Json;
use cgp::prelude::*;

use crate::interfaces::{ApiHandler, ApiHandlerComponent};

pub struct ResponseToJson<InHandler>(pub PhantomData<InHandler>);

#[cgp_impl(ResponseToJson<InHandler>)]
impl<Api, InHandler> ApiHandler<Api>
where
    Self: HasErrorType,
    InHandler: ApiHandler<Self, Api>,
{
    type Request = InHandler::Request;

    type Response = Json<InHandler::Response>;

    async fn handle_api(
        &self,
        api: PhantomData<Api>,
        request: Self::Request,
    ) -> Result<Self::Response, Self::Error> {
        let response = InHandler::handle_api(self, api, request).await?;
        Ok(Json(response))
    }
}
