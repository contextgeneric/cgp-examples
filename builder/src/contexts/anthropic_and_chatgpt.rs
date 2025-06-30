use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::extra::dispatch::BuildAndMergeOutputs;
use cgp::extra::handler::CanHandle;
use cgp::prelude::*;
use cgp_error_anyhow::{Error, RaiseAnyhowError, UseAnyhowError};
use reqwest::Client;
use rig::agent::Agent;
use rig::providers::{anthropic, openai};
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::contexts::anthropic::AnthropicApp;
use crate::contexts::app::App;
use crate::providers::{
    BuildDefaultAnthropicClient, BuildHttpClient, BuildOpenAiClient, BuildSqliteClient,
};

#[cgp_context]
#[derive(HasField, HasFields, BuildField)]
pub struct AnthropicAndChatGptApp {
    pub sqlite_pool: SqlitePool,
    pub http_client: Client,
    pub anthropic_client: anthropic::Client,
    pub anthropic_agent: Agent<anthropic::completion::CompletionModel>,
    pub open_ai_client: openai::Client,
    pub open_ai_agent: Agent<openai::CompletionModel>,
}

#[cgp_context]
#[derive(HasField, Deserialize)]
pub struct AnthropicAndChatGptAppBuilder {
    pub db_options: String,
    pub db_journal_mode: String,
    pub http_user_agent: String,
    pub anthropic_key: String,
    pub open_ai_key: String,
    pub open_ai_model: String,
    pub llm_preamble: String,
}

pub struct BuildAnthroicAndChatGptApp;

pub struct BuildChatGptApp;

pub struct BuildAnthropicApp;

delegate_components! {
    AnthropicAndChatGptAppBuilderComponents {
        ErrorTypeProviderComponent:
            UseAnyhowError,
        ErrorRaiserComponent:
            RaiseAnyhowError,
        HandlerComponent:
            UseDelegate<new BuilderHandlers {
                BuildAnthroicAndChatGptApp:
                BuildAndMergeOutputs<
                        AnthropicAndChatGptApp,
                        Product![
                            BuildSqliteClient,
                            BuildHttpClient,
                            BuildDefaultAnthropicClient,
                            BuildOpenAiClient,
                        ]>,
                BuildChatGptApp:
                    BuildAndMergeOutputs<
                        App,
                        Product![
                            BuildSqliteClient,
                            BuildHttpClient,
                            BuildOpenAiClient,
                        ]>,
                BuildAnthropicApp:
                    BuildAndMergeOutputs<
                        AnthropicApp,
                        Product![
                            BuildSqliteClient,
                            BuildHttpClient,
                            BuildDefaultAnthropicClient,
                        ]>,
            }>,
    }
}

check_components! {
    CanUseAnthropicAndChatGptAppBuilder for AnthropicAndChatGptAppBuilder {
        HandlerComponent: [
            (BuildAnthroicAndChatGptApp, ()),
            (BuildChatGptApp, ()),
            (BuildAnthropicApp, ()),
        ],
    }
}

pub async fn main() -> Result<(), Error> {
    let builder = AnthropicAndChatGptAppBuilder {
        db_options: "file:./db.sqlite".to_owned(),
        db_journal_mode: "WAL".to_owned(),
        http_user_agent: "SUPER_AI_AGENT".to_owned(),
        anthropic_key: "1234567890".to_owned(),
        open_ai_key: "1234567890".to_owned(),
        open_ai_model: "gpt-4o".to_owned(),
        llm_preamble: "You are a helpful assistant".to_owned(),
    };

    let _chat_gpt_app: App = builder.handle(PhantomData::<BuildChatGptApp>, ()).await?;
    let _anthropic_app: AnthropicApp = builder.handle(PhantomData::<BuildAnthropicApp>, ()).await?;
    let _anthropic_and_chat_gpt_app: AnthropicAndChatGptApp = builder
        .handle(PhantomData::<BuildAnthroicAndChatGptApp>, ())
        .await?;

    /* Call methods on the apps here */

    Ok(())
}
