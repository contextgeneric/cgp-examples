use core::fmt::Display;

use anyhow::anyhow;
use axum::http::StatusCode;
use cgp::prelude::*;

use crate::interfaces::{
    ErrBadRequest, ErrInternal, ErrNotFound, ErrUnauthorized, HttpErrorRaiser,
    HttpErrorRaiserComponent,
};
use crate::types::AppError;

/// Maps a status-code marker type to its concrete `StatusCode`, so a provider can turn a
/// marker such as `ErrNotFound` into `404` at the type level with no runtime match.
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

// A provider for `CanRaiseHttpError` covering any `Display` detail: it stamps the status
// code from the `Code` marker and formats the detail into an `AppError`. The
// `#[use_type(HasErrorType.{Error = AppError})]` equality form pins the abstract error to
// the concrete `AppError`, so the body can build one directly. `new` also declares the struct.
#[cgp_impl(new DisplayHttpError)]
#[use_type(HasErrorType.{Error = AppError})]
impl<Code, Detail> HttpErrorRaiser<Code, Detail>
where
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

// A sibling provider for details that are already convertible into `anyhow::Error`: it
// forwards the detail unchanged rather than formatting it, preserving the error chain. The
// wiring chooses between this and `DisplayHttpError` per detail type.
#[cgp_impl(new HandleHttpErrorWithAnyhow)]
#[use_type(HasErrorType.{Error = AppError})]
impl<Code, Detail> HttpErrorRaiser<Code, Detail>
where
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
