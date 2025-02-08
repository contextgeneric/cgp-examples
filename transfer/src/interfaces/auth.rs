use cgp::prelude::*;

use crate::interfaces::{HasHashedPasswordType, HasPasswordType, HasUserIdType};

#[cgp_auto_getter]
pub trait HasLoggedInUser: HasUserIdType {
    fn logged_in_user(&self) -> &Option<Self::UserId>;
}

#[cgp_auto_getter]
pub trait HasLoggedInUserMut: HasUserIdType {
    fn logged_in_user(&mut self) -> &mut Option<Self::UserId>;
}

#[cgp_component {
    provider: PasswordChecker,
}]
pub trait CanCheckPassword: HasPasswordType + HasHashedPasswordType {
    fn check_password(password: &Self::Password, hashed_password: &Self::HashedPassword) -> bool;
}

#[cgp_component {
    provider: UserHashedPasswordQuerier,
}]
#[async_trait]
pub trait CanQueryUserHashedPassword:
    HasUserIdType + HasHashedPasswordType + HasAsyncErrorType
{
    async fn query_user_hashed_password(
        &self,
        user_id: &Self::UserId,
    ) -> Result<Option<Self::HashedPassword>, Self::Error>;
}
