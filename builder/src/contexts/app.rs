use cgp::prelude::*;
use reqwest::Client;
use rig::providers::openai;
use sqlx::SqlitePool;

#[cgp_context]
#[derive(HasField, HasFields, BuildField)]
pub struct App {
    pub open_ai_client: openai::Client,
    pub open_ai_model: openai::CompletionModel,
    pub sqlite_pool: SqlitePool,
    pub http_client: Client,
}
