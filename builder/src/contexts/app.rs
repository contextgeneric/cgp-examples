use core::time::Duration;

use cgp::prelude::*;
use cgp_error_anyhow::Error;
use reqwest::Client;
use rig::agent::Agent;
use rig::client::CompletionClient;
use rig::providers::openai;
use sqlx::SqlitePool;

#[cgp_context]
#[derive(HasField, HasFields, BuildField)]
pub struct App {
    pub open_ai_client: openai::Client,
    pub open_ai_agent: Agent<openai::CompletionModel>,
    pub sqlite_pool: SqlitePool,
    pub http_client: Client,
}

impl App {
    pub async fn new(
        db_path: &str,
        http_user_agent: &str,
        open_ai_key: &str,
        open_ai_model: &str,
    ) -> Result<Self, Error> {
        let http_client = Client::builder()
            .user_agent(http_user_agent)
            .connect_timeout(Duration::from_secs(5))
            .build()?;

        let sqlite_pool = SqlitePool::connect(db_path).await?;

        let open_ai_client = openai::Client::new(open_ai_key);
        let open_ai_agent = open_ai_client.agent(open_ai_model).build();

        Ok(Self {
            open_ai_client,
            open_ai_agent,
            sqlite_pool,
            http_client,
        })
    }
}
