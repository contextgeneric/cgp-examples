use core::marker::PhantomData;

use cgp::prelude::*;

use crate::interfaces::*;

pub struct UseBasicAuth<InHandler>(pub PhantomData<InHandler>);

#[cgp_auto_getter]
pub trait HasBasicAuthenticationHeader<App>
where
    App: HasUserIdType + HasPasswordType,
{
    fn basic_authentication_header(&self) -> &Option<(App::UserId, App::Password)>;
}

impl<App, Api, InHandler> ApiHandler<App, Api> for UseBasicAuth<InHandler>
where
    App: HasLoggedInUserMut + CanQueryUserHashedPassword + CanCheckPassword,
    InHandler: ApiHandler<App, Api>,
    InHandler::Request: HasBasicAuthenticationHeader<App>,
    App::UserId: Clone,
{
    type Request = InHandler::Request;

    type Response = InHandler::Response;

    async fn handle_api(
        app: &mut App,
        request: Self::Request,
    ) -> Result<Self::Response, <App>::Error> {
        if app.logged_in_user().is_none() {
            if let Some((user_id, password)) = request.basic_authentication_header() {
                let m_hashed_password = app.query_user_hashed_password(user_id).await?;

                if let Some(hashed_password) = m_hashed_password {
                    if App::check_password(password, &hashed_password) {
                        *app.logged_in_user() = Some(user_id.clone());
                    }
                }
            }
        }

        InHandler::handle_api(app, request).await
    }
}
