use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::extra::dispatch::BuildAndMergeOutputs;
use cgp::prelude::*;
use cgp_error_anyhow::{RaiseAnyhowError, UseAnyhowError};
use serde::Deserialize;

use crate::contexts::App;
use crate::providers::{BuildHttpClient, BuildOpenAiClient, BuildSqliteClient};

#[cgp_context]
#[derive(HasField, Deserialize)]
pub struct FullAppBuilder {
    pub db_path: String,
    pub http_user_agent: String,
    pub open_ai_key: String,
    pub open_ai_model: String,
    pub llm_preamble: String,
}

delegate_components! {
    FullAppBuilderComponents {
        ErrorTypeProviderComponent:
            UseAnyhowError,
        ErrorRaiserComponent:
            RaiseAnyhowError,
        HandlerComponent:
            BuildAndMergeOutputs<
                App,
                Product![
                    BuildSqliteClient,
                    BuildHttpClient,
                    BuildOpenAiClient,
                ]>,
    }
}

check_components! {
    CanUseFullAppBuilder for FullAppBuilder {
        HandlerComponent: ((), ()),
    }
}
