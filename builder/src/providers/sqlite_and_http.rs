use cgp::prelude::*;
use reqwest::Client;
use sqlx::SqlitePool;

use crate::providers::HasSqlitePath;

#[derive(HasField, HasFields, BuildField)]
pub struct SqliteAndHttpClient {
    pub sqlite_pool: SqlitePool,

    pub http_client: Client,
}

#[cgp_impl(new BuildDefaultSqliteAndHttpClient)]
impl<Build, Code, Input> Handler<Code, Input> for Build
where
    Build: HasSqlitePath + CanRaiseError<sqlx::Error>,
{
    type Output = SqliteAndHttpClient;

    async fn handle(
        build: &Build,
        _code: PhantomData<Code>,
        _input: Input,
    ) -> Result<Self::Output, Build::Error> {
        let sqlite_pool = SqlitePool::connect(build.db_path())
            .await
            .map_err(Build::raise_error)?;

        let http_client = Client::new();

        Ok(SqliteAndHttpClient {
            sqlite_pool,
            http_client,
        })
    }
}
