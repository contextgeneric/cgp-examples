use cgp::prelude::*;

use crate::interfaces::*;

#[cgp_auto_getter]
pub trait HasBasicAuthHeader<App>
where
    App: HasUserIdType + HasPasswordType,
{
    fn basic_auth_header(&self) -> &Option<(App::UserId, App::Password)>;
}

#[cgp_impl(new UseBasicAuth<InHandler>)]
impl<Api, InHandler> ApiHandler<Api>
where
    Self: CanQueryUserHashedPassword + CanCheckPassword,
    InHandler::Request: HasLoggedInUserMut<Self> + HasBasicAuthHeader<Self>,
    InHandler: ApiHandler<Self, Api>,
    Self::UserId: Clone,
{
    type Request = InHandler::Request;

    type Response = InHandler::Response;

    async fn handle_api(
        &self,
        api: PhantomData<Api>,
        mut request: Self::Request,
    ) -> Result<Self::Response, Self::Error> {
        if request.logged_in_user().is_none()
            && let Some((user_id, password)) = request.basic_auth_header()
        {
            let m_hashed_password = self.query_user_hashed_password(user_id).await?;

            if let Some(hashed_password) = m_hashed_password
                && Self::check_password(password, &hashed_password)
            {
                *request.logged_in_user() = Some(user_id.clone());
            }
        }

        InHandler::handle_api(self, api, request).await
    }
}
