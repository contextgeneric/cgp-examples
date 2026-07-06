use cgp::prelude::*;

#[cgp_component(HttpErrorRaiser)]
#[prefix(@app.error in DefaultNamespace)]
#[use_type(HasErrorType.Error)]
pub trait CanRaiseHttpError<Code, Detail> {
    fn raise_http_error(_code: Code, detail: Detail) -> Error;
}

pub struct ErrUnauthorized;

pub struct ErrBadRequest;

pub struct ErrNotFound;

pub struct ErrInternal;
