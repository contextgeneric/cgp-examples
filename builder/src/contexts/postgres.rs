use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::extra::dispatch::BuildAndMergeOutputs;
use cgp::prelude::*;
use cgp_error_anyhow::{RaiseAnyhowError, UseAnyhowError};
use reqwest::Client;
use rig::agent::Agent;
use rig::providers::openai;
use serde::Deserialize;
use sqlx::PgPool;

use crate::providers::{BuildHttpClient, BuildOpenAiClient, BuildPostgresClient};

#[derive(HasField, HasFields, BuildField)]
pub struct App {
    pub postgres_pool: PgPool,
    pub http_client: Client,
    pub open_ai_client: openai::Client,
    pub open_ai_agent: Agent<openai::CompletionModel>,
}

#[derive(HasField, Deserialize)]
pub struct AppBuilder {
    pub postgres_url: String,
    pub http_user_agent: String,
    pub open_ai_key: String,
    pub open_ai_model: String,
    pub llm_preamble: String,
}

delegate_components! {
    AppBuilder {
        ErrorTypeProviderComponent:
            UseAnyhowError,
        ErrorRaiserComponent:
            RaiseAnyhowError,
        HandlerComponent:
            BuildAndMergeOutputs<
                App,
                Product![
                    BuildPostgresClient,
                    BuildHttpClient,
                    BuildOpenAiClient,
                ]>,
    }
}

check_components! {
    CanUseAppBuilder for AppBuilder {
        HandlerComponent: ((), ()),
    }
}
