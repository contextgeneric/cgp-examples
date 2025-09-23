use cgp::prelude::*;

use crate::interfaces::*;

#[cgp_auto_getter]
pub trait HasBasicAuthHeader<App>
where
    App: HasUserIdType + HasPasswordType,
{
    fn basic_auth_header(&self) -> &Option<(App::UserId, App::Password)>;
}

#[cgp_new_provider]
impl<App, Api, InHandler> ApiHandler<App, Api> for UseBasicAuth<InHandler>
where
    App: CanQueryUserHashedPassword + CanCheckPassword,
    InHandler::Request: HasLoggedInUserMut<App> + HasBasicAuthHeader<App>,
    InHandler: ApiHandler<App, Api>,
    App::UserId: Clone,
{
    type Request = InHandler::Request;

    type Response = InHandler::Response;

    async fn handle_api(
        app: &App,
        api: PhantomData<Api>,
        mut request: Self::Request,
    ) -> Result<Self::Response, App::Error> {
        if request.logged_in_user().is_none()
            && let Some((user_id, password)) = request.basic_auth_header()
        {
            let m_hashed_password = app.query_user_hashed_password(user_id).await?;

            if let Some(hashed_password) = m_hashed_password
                && App::check_password(password, &hashed_password)
            {
                *request.logged_in_user() = Some(user_id.clone());
            }
        }

        InHandler::handle_api(app, api, request).await
    }
}
