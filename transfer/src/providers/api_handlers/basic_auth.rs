use cgp::prelude::*;

use crate::interfaces::*;

// Getter for the optional Basic-auth credentials on a request. Implemented on the request
// struct (not the app), so it is a getter trait: the auth wrapper requires it as
// `InHandler::Request: HasBasicAuthHeader<Self>`.
#[cgp_auto_getter]
pub trait HasBasicAuthHeader<App>
where
    App: HasUserIdType + HasPasswordType,
{
    fn basic_auth_header(&self) -> &Option<(App::UserId, App::Password)>;
}

// A higher-order `ApiHandler` provider that authenticates, then delegates. If no user is
// logged in yet, it resolves the Basic-auth header into a user, verifies the password via the
// wired capabilities (`#[uses(...)]`), records the user on the request, and calls the inner
// handler. `#[use_provider(InHandler: ApiHandler<Api>)]` supplies the inner handler's hidden
// context argument, so the body calls it as `InHandler::handle_api(self, ...)`.
#[cgp_impl(new UseBasicAuth<InHandler>)]
#[uses(CanQueryUserHashedPassword, CanCheckPassword)]
#[use_type(HasUserIdType.UserId, HasErrorType.Error)]
#[use_provider(InHandler: ApiHandler<Api>)]
impl<Api, InHandler> ApiHandler<Api>
where
    InHandler::Request: HasLoggedInUserMut<Self> + HasBasicAuthHeader<Self>,
    Self::UserId: Clone,
{
    type Request = InHandler::Request;

    type Response = InHandler::Response;

    async fn handle_api(
        &self,
        api: PhantomData<Api>,
        mut request: Self::Request,
    ) -> Result<Self::Response, Error> {
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
