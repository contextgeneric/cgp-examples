use core::fmt::Display;

use anyhow::anyhow;
use axum::http::StatusCode;
use cgp::prelude::*;

use crate::interfaces::{
    ErrBadRequest, ErrInternal, ErrNotFound, ErrUnauthorized, HttpErrorRaiser,
    HttpErrorRaiserComponent,
};
use crate::types::AppError;

pub trait IsStatusCode {
    fn status_code() -> StatusCode;
}

impl IsStatusCode for ErrUnauthorized {
    fn status_code() -> StatusCode {
        StatusCode::UNAUTHORIZED
    }
}

impl IsStatusCode for ErrBadRequest {
    fn status_code() -> StatusCode {
        StatusCode::BAD_REQUEST
    }
}

impl IsStatusCode for ErrNotFound {
    fn status_code() -> StatusCode {
        StatusCode::NOT_FOUND
    }
}

impl IsStatusCode for ErrInternal {
    fn status_code() -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

pub struct DisplayHttpError;

#[cgp_impl(DisplayHttpError)]
impl<Context, Code, Detail> HttpErrorRaiser<Code, Detail> for Context
where
    Context: HasErrorType<Error = AppError>,
    Code: IsStatusCode,
    Detail: Display,
{
    fn raise_http_error(_code: Code, detail: Detail) -> AppError {
        AppError {
            status_code: Code::status_code(),
            detail: anyhow!("{detail}"),
        }
    }
}

pub struct HandleHttpErrorWithAnyhow;

#[cgp_impl(HandleHttpErrorWithAnyhow)]
impl<Context, Code, Detail> HttpErrorRaiser<Code, Detail> for Context
where
    Context: HasErrorType<Error = AppError>,
    Code: IsStatusCode,
    anyhow::Error: From<Detail>,
{
    fn raise_http_error(_code: Code, detail: Detail) -> AppError {
        AppError {
            status_code: Code::status_code(),
            detail: detail.into(),
        }
    }
}
