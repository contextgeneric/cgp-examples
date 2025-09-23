use core::fmt::Display;

use cgp::prelude::*;

#[cgp_type]
pub trait HasUserIdType {
    type UserId: Display;
}

#[cgp_type]
pub trait HasQuantityType {
    type Quantity: Display;
}

#[cgp_type]
pub trait HasCurrencyType {
    type Currency: Display;
}

#[cgp_type]
pub trait HasPasswordType {
    type Password;
}

#[cgp_type]
pub trait HasHashedPasswordType {
    type HashedPassword;
}
