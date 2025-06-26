use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::extra::dispatch::BuildAndMergeOutputs;
use cgp::prelude::*;
use cgp_error_anyhow::{RaiseAnyhowError, UseAnyhowError};
use serde::Deserialize;

use crate::contexts::App;
use crate::providers::{BuildHttpClient, BuildOpenAiClient, BuildSqliteClient};

#[cgp_context]
#[derive(HasField, Deserialize)]
pub struct AppBuilder {
    pub open_ai_key: String,
    pub open_ai_model: String,
    pub db_path: String,
    pub http_user_agent: String,
}

delegate_components! {
    AppBuilderComponents {
        ErrorTypeProviderComponent:
            UseAnyhowError,
        ErrorRaiserComponent:
            RaiseAnyhowError,
        HandlerComponent:
            BuildAndMergeOutputs<
                App,
                Product![
                    BuildOpenAiClient,
                    BuildSqliteClient,
                    BuildHttpClient,
                ]>,
    }
}

check_components! {
    CanUseAppBuilder for AppBuilder {
        HandlerComponent: ((), ()),
    }
}
