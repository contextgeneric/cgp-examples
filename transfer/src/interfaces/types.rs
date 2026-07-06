use core::fmt::Display;

use cgp::prelude::*;

// The domain's abstract types. Each `#[cgp_type]` declares a component whose only
// content is one associated type, so handlers can name `UserId`/`Quantity`/`Currency`
// generically and a context picks the concrete type at wiring time (via `UseType<T>`).
// The `#[prefix(... in DefaultNamespace)]` files each component under a dotted path so
// the wiring can address it by that path; see the namespaces module.

// A context's abstract user-identifier type, kept `Display` so it can appear in errors.
#[cgp_type]
#[prefix(@app.auth.types in DefaultNamespace)]
pub trait HasUserIdType {
    type UserId: Display;
}

// A context's abstract money-amount type.
#[cgp_type]
#[prefix(@app.finance.types in DefaultNamespace)]
pub trait HasQuantityType {
    type Quantity: Display;
}

// A context's abstract currency type.
#[cgp_type]
#[prefix(@app.finance.types in DefaultNamespace)]
pub trait HasCurrencyType {
    type Currency: Display;
}

// A context's abstract cleartext-password type (supplied on a login request).
#[cgp_type]
#[prefix(@app.auth.types in DefaultNamespace)]
pub trait HasPasswordType {
    type Password;
}

// A context's abstract stored-password type, compared against `Password` to authenticate.
#[cgp_type]
#[prefix(@app.auth.types in DefaultNamespace)]
pub trait HasHashedPasswordType {
    type HashedPassword;
}
