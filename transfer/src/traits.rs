use core::fmt::Display;

use cgp::prelude::*;

cgp_type!( UserId: Async + Display );

cgp_type!( Quantity: Async + Display );

cgp_type!( Currency: Async + Display );

cgp_type!( Password: Async );

cgp_type!( HashedPassword: Async );

#[cgp_auto_getter]
pub trait HasLoggedInUser: HasUserIdType {
    fn logged_in_user(&self) -> &Option<Self::UserId>;
}

#[cgp_auto_getter]
pub trait HasLoggedInUserMut: HasUserIdType {
    fn logged_in_user(&mut self) -> &mut Option<Self::UserId>;
}

#[cgp_component {
    provider: ApiHandler,
}]
#[async_trait]
pub trait CanHandleApi<Api>: HasAsyncErrorType {
    type Request: Async;

    type Response: Async;

    async fn handle_api(&mut self, request: &Self::Request) -> Result<Self::Response, Self::Error>;
}

#[cgp_component {
    provider: UserAuthenticator,
}]
#[async_trait]
pub trait CanAuthenticateUser: HasAsyncErrorType {
    async fn authenticate_user(&mut self) -> Result<(), Self::Error>;
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

#[cgp_component {
    provider: UserBalanceQuerier,
}]
#[async_trait]
pub trait CanQueryCurrentUserBalance:
    HasUserIdType + HasCurrencyType + HasQuantityType + HasAsyncErrorType
{
    async fn get_current_user_balance(
        &self,
        currency: &Self::Currency,
    ) -> Result<Self::Quantity, Self::Error>;
}

#[cgp_component {
    provider: MoneyTransferrer,
}]
#[async_trait]
pub trait CanTransferMoney:
    HasUserIdType + HasCurrencyType + HasQuantityType + HasAsyncErrorType
{
    async fn transfer_money(
        &self,
        recipient: &Self::UserId,
        currency: &Self::Currency,
        quantity: &Self::Quantity,
    ) -> Result<(), Self::Error>;
}
