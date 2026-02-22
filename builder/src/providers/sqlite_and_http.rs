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
impl<Code, Input> Handler<Code, Input>
where
    Self: HasSqlitePath + CanRaiseError<sqlx::Error>,
{
    type Output = SqliteAndHttpClient;

    async fn handle(
        &self,
        _code: PhantomData<Code>,
        _input: Input,
    ) -> Result<Self::Output, Self::Error> {
        let sqlite_pool = SqlitePool::connect(self.db_path())
            .await
            .map_err(Self::raise_error)?;

        let http_client = Client::new();

        Ok(SqliteAndHttpClient {
            sqlite_pool,
            http_client,
        })
    }
}
