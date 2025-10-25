use cgp::core::component::UseDelegate;
use cgp::prelude::*;

#[cgp_component(HttpErrorRaiser)]
pub trait CanRaiseHttpError<Code, Detail>: HasErrorType {
    fn raise_http_error(_code: Code, detail: Detail) -> Self::Error;
}

pub struct ErrUnauthorized;

pub struct ErrBadRequest;

pub struct ErrNotFound;

pub struct ErrInternal;

#[cgp_impl(UseDelegate<Components>)]
impl<Context, Components, Code, Detail> HttpErrorRaiser<Code, Detail> for Context
where
    Context: HasErrorType,
    Components: DelegateComponent<(Code, Detail)>,
    Components::Delegate: HttpErrorRaiser<Context, Code, Detail>,
{
    fn raise_http_error(code: Code, detail: Detail) -> Context::Error {
        Components::Delegate::raise_http_error(code, detail)
    }
}
