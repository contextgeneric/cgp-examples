use core::marker::PhantomData;

use axum::Json;
use cgp::prelude::*;

use crate::interfaces::{ApiHandler, ApiHandlerComponent};

pub struct ResponseToJson<InHandler>(pub PhantomData<InHandler>);

#[cgp_provider]
impl<App, Api, InHandler> ApiHandler<App, Api> for ResponseToJson<InHandler>
where
    App: HasAsyncErrorType,
    InHandler: ApiHandler<App, Api>,
{
    type Request = InHandler::Request;

    type Response = Json<InHandler::Response>;

    async fn handle_api(app: &App, request: Self::Request) -> Result<Self::Response, App::Error> {
        let response = InHandler::handle_api(app, request).await?;
        Ok(Json(response))
    }
}
