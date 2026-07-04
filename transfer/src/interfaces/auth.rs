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
#[use_type(HasPasswordType.Password)]
#[use_type(HasHashedPasswordType.HashedPassword)]
pub trait CanCheckPassword {
    fn check_password(password: &Password, hashed_password: &HashedPassword) -> bool;
}

#[cgp_component(UserHashedPasswordQuerier)]
#[async_trait]
#[use_type(HasUserIdType.UserId)]
#[use_type(HasHashedPasswordType.HashedPassword)]
#[use_type(HasErrorType.Error)]
pub trait CanQueryUserHashedPassword {
    async fn query_user_hashed_password(
        &self,
        user_id: &UserId,
    ) -> Result<Option<HashedPassword>, Error>;
}
