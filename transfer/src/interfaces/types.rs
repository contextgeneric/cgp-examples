use core::fmt::Display;

use cgp::prelude::*;

#[cgp_type]
#[prefix(@app.auth.types in DefaultNamespace)]
pub trait HasUserIdType {
    type UserId: Display;
}

#[cgp_type]
#[prefix(@app.finance.types in DefaultNamespace)]
pub trait HasQuantityType {
    type Quantity: Display;
}

#[cgp_type]
#[prefix(@app.finance.types in DefaultNamespace)]
pub trait HasCurrencyType {
    type Currency: Display;
}

#[cgp_type]
#[prefix(@app.auth.types in DefaultNamespace)]
pub trait HasPasswordType {
    type Password;
}

#[cgp_type]
#[prefix(@app.auth.types in DefaultNamespace)]
pub trait HasHashedPasswordType {
    type HashedPassword;
}
