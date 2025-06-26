use core::str::FromStr;
use core::time::Duration;

use cgp::prelude::*;
use cgp_error_anyhow::Error;
use reqwest::Client;
use rig::agent::Agent;
use rig::client::{CompletionClient, ProviderClient};
use rig::providers::openai;
use sqlx::SqlitePool;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};

#[cgp_context]
#[derive(HasField, HasFields, BuildField)]
pub struct App {
    pub sqlite_pool: SqlitePool,
    pub http_client: Client,
    pub open_ai_client: openai::Client,
    pub open_ai_agent: Agent<openai::CompletionModel>,
}

impl App {
    pub async fn new(
        db_options: &str,
        http_user_agent: &str,
        open_ai_key: &str,
        open_ai_model: &str,
        llm_preamble: &str,
    ) -> Result<Self, Error> {
        let db_options =
            SqliteConnectOptions::from_str(db_options)?.journal_mode(SqliteJournalMode::Wal);

        let sqlite_pool = SqlitePool::connect_with(db_options).await?;

        let http_client = Client::builder()
            .user_agent(http_user_agent)
            .connect_timeout(Duration::from_secs(5))
            .build()?;

        let open_ai_client = openai::Client::new(open_ai_key);
        let open_ai_agent = open_ai_client
            .agent(open_ai_model)
            .preamble(llm_preamble)
            .build();

        Ok(Self {
            open_ai_client,
            open_ai_agent,
            sqlite_pool,
            http_client,
        })
    }

    pub async fn new_with_default(db_path: &str) -> Result<Self, Error> {
        let http_client = Client::new();
        let sqlite_pool = SqlitePool::connect(db_path).await?;
        let open_ai_client = openai::Client::from_env();
        let open_ai_agent = open_ai_client.agent("gpt-4o").build();

        Ok(Self {
            http_client,
            sqlite_pool,
            open_ai_client,
            open_ai_agent,
        })
    }
}
