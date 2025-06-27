use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::extra::dispatch::BuildAndMergeOutputs;
use cgp::prelude::*;
use cgp_error_anyhow::{RaiseAnyhowError, UseAnyhowError};
use reqwest::Client;
use rig::agent::Agent;
use rig::providers::anthropic;
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::providers::{BuildDefaultAnthropicClient, BuildHttpClient, BuildSqliteClient};

#[cgp_context]
#[derive(HasField, HasFields, BuildField)]
pub struct App {
    pub sqlite_pool: SqlitePool,
    pub http_client: Client,
    pub anthropic_client: anthropic::Client,
    pub anthropic_agent: Agent<anthropic::completion::CompletionModel>,
}

#[cgp_context]
#[derive(HasField, Deserialize)]
pub struct AppBuilder {
    pub db_options: String,
    pub db_journal_mode: String,
    pub http_user_agent: String,
    pub anthropic_key: String,
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
                    BuildSqliteClient,
                    BuildHttpClient,
                    BuildDefaultAnthropicClient,
                ]>,
    }
}

check_components! {
    CanUseAppBuilder for AppBuilder {
        HandlerComponent: ((), ()),
    }
}
