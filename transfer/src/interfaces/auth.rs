use cgp::prelude::*;

use crate::interfaces::{HasHashedPasswordType, HasPasswordType, HasUserIdType};

// Read the already-authenticated user out of a request. `#[cgp_auto_getter]` turns this
// getter trait into a blanket impl over any type with a matching `logged_in_user` field,
// so it is implemented on the request struct, not the app context — which is why it is a
// getter trait rather than an implicit argument (those read only from `self`).
#[cgp_auto_getter]
pub trait HasLoggedInUser<App>
where
    App: HasUserIdType,
{
    fn logged_in_user(&self) -> &Option<App::UserId>;
}

// The mutable counterpart, used by the auth wrapper to record the user it just logged in.
#[cgp_auto_getter]
pub trait HasLoggedInUserMut<App>
where
    App: HasUserIdType,
{
    fn logged_in_user(&mut self) -> &mut Option<App::UserId>;
}

// Capability to compare a cleartext password against a stored one. A component so a
// deployment can swap the plain-equality mock for real password hashing without touching
// callers. `#[use_type]` imports the abstract password types so they read as bare names.
#[cgp_component(PasswordChecker)]
#[prefix(@app.auth in DefaultNamespace)]
#[use_type(HasPasswordType.Password, HasHashedPasswordType.HashedPassword)]
pub trait CanCheckPassword {
    fn check_password(password: &Password, hashed_password: &HashedPassword) -> bool;
}

// Capability to look up a user's stored password. Abstract over the backend: the mock reads
// an in-memory map, a real deployment would query a database, and the auth wrapper depends
// only on this trait.
#[cgp_component(UserHashedPasswordQuerier)]
#[prefix(@app.auth in DefaultNamespace)]
#[async_trait]
#[use_type(HasUserIdType.UserId, HasHashedPasswordType.HashedPassword, HasErrorType.Error)]
pub trait CanQueryUserHashedPassword {
    async fn query_user_hashed_password(
        &self,
        user_id: &UserId,
    ) -> Result<Option<HashedPassword>, Error>;
}
