use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::extra::dispatch::BuildAndMergeOutputs;
use cgp::prelude::*;
use cgp_error_anyhow::{RaiseAnyhowError, UseAnyhowError};
use serde::Deserialize;

use crate::contexts::App;
use crate::providers::{BuildDefaultHttpClient, BuildDefaultOpenAiClient, BuildSqliteClient};

#[cgp_context]
#[derive(HasField, Deserialize)]
pub struct MinAppBuilder {
    pub db_path: String,
}

delegate_components! {
    MinAppBuilderComponents {
        ErrorTypeProviderComponent:
            UseAnyhowError,
        ErrorRaiserComponent:
            RaiseAnyhowError,
        HandlerComponent:
            BuildAndMergeOutputs<
                App,
                Product![
                    BuildSqliteClient,
                    BuildDefaultHttpClient,
                    BuildDefaultOpenAiClient,
                ]>,
    }
}

check_components! {
    CanUseMinAppBuilder for MinAppBuilder {
        HandlerComponent: ((), ()),
    }
}
