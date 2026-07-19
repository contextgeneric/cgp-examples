use cgp::core::error::ErrorTypeProviderComponent;
use cgp::prelude::*;

use crate::interfaces::*;
use crate::providers::DisplayHttpError;
use crate::types::{AppError, DemoCurrency};

// The mock application's namespace. It inherits every default `DefaultNamespace`
// entry — which is how the mock providers register themselves through the
// `#[default_impl(... in MockNamespace)]` attributes in `providers/mocked.rs` —
// and additionally wires the pieces that have no `#[cgp_impl]` block of their own
// to attach a `#[default_impl]` to: the concrete error type, the app's HTTP
// error-raising dispatch, and the abstract type choices.
cgp_namespace! {
    new MockNamespace: DefaultNamespace {
        @cgp.core.error.ErrorTypeProviderComponent:
            UseType<AppError>,

        @app.error.HttpErrorRaiserComponent.<Code> Code.String:
            DisplayHttpError,

        @app.auth.types.{
            UserIdTypeProviderComponent,
            PasswordTypeProviderComponent,
            HashedPasswordTypeProviderComponent,
        }:
            UseType<String>,
        @app.finance.types.QuantityTypeProviderComponent:
            UseType<u64>,
        @app.finance.types.CurrencyTypeProviderComponent:
            UseType<DemoCurrency>,
    }
}
