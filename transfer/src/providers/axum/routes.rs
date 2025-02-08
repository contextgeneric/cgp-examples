use core::marker::PhantomData;
use std::sync::Arc;

use axum::extract::{FromRequest, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;
use cgp::prelude::HasErrorType;

use crate::interfaces::CanHandleApi;
use crate::types::AppError;

pub struct GetMethod;

pub struct PostMethod;

pub trait CanAddRoute<App, Api, Method> {
    fn add_route(self, _tag: PhantomData<(Api, Method)>, path: &str) -> Self;
}

pub fn handle_api_error(err: AppError) -> (StatusCode, String) {
    let status_code = err.status_code;

    let detail = err.detail.to_string();

    (status_code, detail)
}

impl<App, Api> CanAddRoute<App, Api, GetMethod> for Router<Arc<App>>
where
    App: HasErrorType<Error = AppError> + CanHandleApi<Api>,
    App::Request: FromRequest<Arc<App>>,
    App::Response: IntoResponse,
{
    fn add_route(self, _tag: PhantomData<(Api, GetMethod)>, path: &str) -> Self {
        self.route(
            path,
            get(
                |(State(app), request): (State<Arc<App>>, App::Request)| async move {
                    app.handle_api(request).await.map_err(handle_api_error)
                },
            ),
        )
    }
}

impl<App, Api> CanAddRoute<App, Api, PostMethod> for Router<Arc<App>>
where
    App: HasErrorType<Error = AppError> + CanHandleApi<Api>,
    App::Request: FromRequest<Arc<App>>,
    App::Response: IntoResponse,
{
    fn add_route(self, _tag: PhantomData<(Api, PostMethod)>, path: &str) -> Self {
        self.route(
            path,
            post(
                |(State(app), request): (State<Arc<App>>, App::Request)| async move {
                    app.handle_api(request).await.map_err(handle_api_error)
                },
            ),
        )
    }
}
