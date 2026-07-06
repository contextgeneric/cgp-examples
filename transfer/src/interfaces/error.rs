use cgp::prelude::*;

// Capability to build the context's abstract error from a status-code marker and a detail
// value, so the mapping from a domain failure to an HTTP status lives in one provider. It
// is generic over the `Code` marker and the `Detail` type; `#[use_type(HasErrorType.Error)]`
// imports the context's shared error type as the return type.
#[cgp_component(HttpErrorRaiser)]
#[prefix(@app.error in DefaultNamespace)]
#[use_type(HasErrorType.Error)]
pub trait CanRaiseHttpError<Code, Detail> {
    fn raise_http_error(_code: Code, detail: Detail) -> Error;
}

/// Status-code markers passed as the `Code` argument of `raise_http_error`. Each is a
/// zero-sized type that a provider maps to a concrete HTTP status (see `providers/error.rs`).
pub struct ErrUnauthorized;

pub struct ErrBadRequest;

pub struct ErrNotFound;

pub struct ErrInternal;
