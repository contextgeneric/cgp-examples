use cgp::prelude::*;

use crate::interfaces::{HasHashedPasswordType, HasPasswordType, HasUserIdType};

#[cgp_auto_getter]
pub trait HasLoggedInUser<App>
where
    App: HasUserIdType,
{
    fn logged_in_user(&self) -> &Option<App::UserId>;
}

#[cgp_auto_getter]
pub trait HasLoggedInUserMut<App>
where
    App: HasUserIdType,
{
    fn logged_in_user(&mut self) -> &mut Option<App::UserId>;
}

#[cgp_component(PasswordChecker)]
pub trait CanCheckPassword: HasPasswordType + HasHashedPasswordType {
    fn check_password(password: &Self::Password, hashed_password: &Self::HashedPassword) -> bool;
}

#[cgp_component(UserHashedPasswordQuerier)]
#[async_trait]
pub trait CanQueryUserHashedPassword: HasUserIdType + HasHashedPasswordType + HasErrorType {
    async fn query_user_hashed_password(
        &self,
        user_id: &Self::UserId,
    ) -> Result<Option<Self::HashedPassword>, Self::Error>;
}
