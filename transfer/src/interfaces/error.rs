use cgp::core::component::UseDelegate;
use cgp::prelude::*;

#[cgp_component {
    provider: HttpErrorRaiser,
}]
pub trait CanRaiseHttpError<Code, Detail>: HasAsyncErrorType {
    fn raise_http_error(detail: Detail) -> Self::Error;
}

pub struct Unauthorized;

pub struct BadRequest;

pub struct InternalServerError;

#[cgp_provider(HttpErrorRaiserComponent)]
impl<Context, Components, Code, Detail> HttpErrorRaiser<Context, Code, Detail>
    for UseDelegate<Components>
where
    Context: HasAsyncErrorType,
    Components: DelegateComponent<(Code, Detail)>,
    Components::Delegate: HttpErrorRaiser<Context, Code, Detail>,
{
    fn raise_http_error(detail: Detail) -> Context::Error {
        Components::Delegate::raise_http_error(detail)
    }
}
