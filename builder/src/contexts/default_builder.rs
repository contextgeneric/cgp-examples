use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::extra::dispatch::BuildAndMergeOutputs;
use cgp::prelude::*;
use cgp_error_anyhow::{RaiseAnyhowError, UseAnyhowError};
use serde::Deserialize;

use crate::contexts::app::App;
use crate::providers::{
    BuildDefaultHttpClient, BuildDefaultOpenAiClient, BuildDefaultSqliteClient,
};

#[cgp_context]
#[derive(HasField, Deserialize)]
pub struct DefaultAppBuilder {
    pub db_path: String,
}

delegate_components! {
    DefaultAppBuilderComponents {
        ErrorTypeProviderComponent:
            UseAnyhowError,
        ErrorRaiserComponent:
            RaiseAnyhowError,
        HandlerComponent:
            BuildAndMergeOutputs<
                App,
                Product![
                    BuildDefaultSqliteClient,
                    BuildDefaultHttpClient,
                    BuildDefaultOpenAiClient,
                ]>,
    }
}

check_components! {
    CanUseMinAppBuilder for DefaultAppBuilder {
        HandlerComponent: ((), ()),
    }
}
