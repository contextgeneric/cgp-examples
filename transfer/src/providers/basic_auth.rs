use core::marker::PhantomData;

use cgp::prelude::*;

use crate::interfaces::*;

pub struct UseBasicAuth<InHandler>(pub PhantomData<InHandler>);

#[cgp_component {
    provider: BasicAuthHeaderExtractor,
}]
#[async_trait]
pub trait CanExtractBasicAuthHeader<Request>: HasUserIdType + HasPasswordType
where
    Request: Async,
{
    async fn extract_basic_authentication_header(
        request: &mut Request,
    ) -> Option<(Self::UserId, Self::Password)>;
}

#[cgp_provider(ApiHandlerComponent)]
impl<App, Api, InHandler> ApiHandler<App, Api> for UseBasicAuth<InHandler>
where
    App: HasLoggedInUserMut
        + CanExtractBasicAuthHeader<InHandler::Request>
        + CanQueryUserHashedPassword
        + CanCheckPassword,
    InHandler: ApiHandler<App, Api>,
    App::UserId: Clone,
{
    type Request = InHandler::Request;

    type Response = InHandler::Response;

    async fn handle_api(
        app: &mut App,
        mut request: Self::Request,
    ) -> Result<Self::Response, <App>::Error> {
        if app.logged_in_user().is_none() {
            if let Some((user_id, password)) =
                App::extract_basic_authentication_header(&mut request).await
            {
                let m_hashed_password = app.query_user_hashed_password(&user_id).await?;

                if let Some(hashed_password) = m_hashed_password {
                    if App::check_password(&password, &hashed_password) {
                        *app.logged_in_user() = Some(user_id.clone());
                    }
                }
            }
        }

        InHandler::handle_api(app, request).await
    }
}
