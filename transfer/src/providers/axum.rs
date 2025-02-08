use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum_extra::TypedHeader;
use cgp::prelude::*;
use headers::authorization::Basic;
use headers::Authorization;

use crate::interfaces::{HasPasswordType, HasUserIdType};
use crate::providers::{BasicAuthHeaderExtractor, BasicAuthHeaderExtractorComponent};

pub trait HasRequestParts {
    fn request_parts(&mut self) -> &mut Option<Parts>;
}

pub struct ExtractWithAxum;

#[cgp_provider(BasicAuthHeaderExtractorComponent)]
impl<App, Request> BasicAuthHeaderExtractor<App, Request> for ExtractWithAxum
where
    App: HasUserIdType + HasPasswordType,
    TypedHeader<Authorization<Basic>>: FromRequestParts<()>,
    Request: Async + HasRequestParts,
    App::UserId: for<'a> From<&'a str>,
    App::Password: for<'a> From<&'a str>,
{
    async fn extract_basic_authentication_header(
        request: &mut Request,
    ) -> Option<(App::UserId, App::Password)> {
        let request_parts = request.request_parts().as_mut()?;
        let TypedHeader(Authorization(basic)) =
            <TypedHeader<Authorization<Basic>>>::from_request_parts(request_parts, &())
                .await
                .ok()?;

        let username = basic.username();
        let password = basic.password();

        Some((username.into(), password.into()))
    }
}
