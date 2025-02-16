use core::fmt::Display;

use cgp::prelude::*;

#[cgp_type]
pub trait HasUserIdType {
    type UserId: Async + Display;
}

#[cgp_type]
pub trait HasQuantityType {
    type Quantity: Async + Display;
}

#[cgp_type]
pub trait HasCurrencyType {
    type Currency: Async + Display;
}

#[cgp_type]
pub trait HasPasswordType {
    type Password: Async;
}

#[cgp_type]
pub trait HasHashedPasswordType {
    type HashedPassword;
}
