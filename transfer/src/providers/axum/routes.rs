use core::marker::PhantomData;
use std::sync::Arc;

use axum::extract::{FromRequest, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;

use crate::interfaces::CanHandleApi;

pub struct GetMethod;

pub struct PostMethod;

pub trait CanAddRoute<App, Api, Method> {
    fn add_route(self, _tag: PhantomData<(Api, Method)>, path: &str) -> Self;
}

impl<App, Api> CanAddRoute<App, Api, GetMethod> for Router<Arc<App>>
where
    App: CanHandleApi<Api>,
    App::Request: FromRequest<Arc<App>>,
    App::Error: IntoResponse,
    App::Response: IntoResponse,
{
    fn add_route(self, _tag: PhantomData<(Api, GetMethod)>, path: &str) -> Self {
        self.route(
            path,
            get(
                |(State(app), request): (State<Arc<App>>, App::Request)| async move {
                    app.handle_api(request).await
                },
            ),
        )
    }
}

impl<App, Api> CanAddRoute<App, Api, PostMethod> for Router<Arc<App>>
where
    App: CanHandleApi<Api>,
    App::Request: FromRequest<Arc<App>>,
    App::Error: IntoResponse,
    App::Response: IntoResponse,
{
    fn add_route(self, _tag: PhantomData<(Api, PostMethod)>, path: &str) -> Self {
        self.route(
            path,
            post(
                |(State(app), request): (State<Arc<App>>, App::Request)| async move {
                    app.handle_api(request).await
                },
            ),
        )
    }
}
