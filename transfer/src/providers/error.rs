use core::fmt::Display;

use anyhow::anyhow;
use axum::http::StatusCode;
use cgp::prelude::*;

use crate::interfaces::{
    BadRequest, HttpErrorRaiser, HttpErrorRaiserComponent, InternalServerError, Unauthorized,
};
use crate::types::AppError;

pub trait IsStatusCode {
    fn status_code() -> StatusCode;
}

impl IsStatusCode for Unauthorized {
    fn status_code() -> StatusCode {
        StatusCode::UNAUTHORIZED
    }
}

impl IsStatusCode for BadRequest {
    fn status_code() -> StatusCode {
        StatusCode::BAD_REQUEST
    }
}

impl IsStatusCode for InternalServerError {
    fn status_code() -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

pub struct DisplayHttpError;

#[cgp_provider(HttpErrorRaiserComponent)]
impl<Context, Code, Detail> HttpErrorRaiser<Context, Code, Detail> for DisplayHttpError
where
    Context: HasAsyncErrorType<Error = AppError>,
    Code: IsStatusCode,
    Detail: Display,
{
    fn raise_http_error(detail: Detail) -> AppError {
        AppError {
            status_code: Code::status_code(),
            detail: anyhow!("{detail}"),
        }
    }
}

pub struct HandleHttpErrorWithAnyhow;

#[cgp_provider(HttpErrorRaiserComponent)]
impl<Context, Code, Detail> HttpErrorRaiser<Context, Code, Detail> for HandleHttpErrorWithAnyhow
where
    Context: HasAsyncErrorType<Error = AppError>,
    Code: IsStatusCode,
    anyhow::Error: From<Detail>,
{
    fn raise_http_error(detail: Detail) -> AppError {
        AppError {
            status_code: Code::status_code(),
            detail: detail.into(),
        }
    }
}
