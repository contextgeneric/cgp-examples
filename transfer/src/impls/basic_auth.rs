use cgp::prelude::*;

use crate::traits::*;

pub struct UseBasicAuth;

#[cgp_auto_getter]
pub trait HasBasicAuthenticationHeader: HasUserIdType + HasPasswordType {
    fn basic_authentication_header(&self) -> &Option<(Self::UserId, Self::Password)>;
}

impl<App> UserAuthenticator<App> for UseBasicAuth
where
    App: HasLoggedInUserMut
        + HasBasicAuthenticationHeader
        + CanQueryUserHashedPassword
        + CanCheckPassword,
    App::UserId: Clone,
{
    async fn authenticate_user(app: &mut App) -> Result<(), App::Error> {
        if app.logged_in_user().is_some() {
            return Ok(());
        }

        if let Some((user_id, password)) = app.basic_authentication_header() {
            let m_hashed_password = app.query_user_hashed_password(user_id).await?;

            if let Some(hashed_password) = m_hashed_password {
                if App::check_password(password, &hashed_password) {
                    *app.logged_in_user() = Some(user_id.clone());
                }
            }
        }

        Ok(())
    }
}
