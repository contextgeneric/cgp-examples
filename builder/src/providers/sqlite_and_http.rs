use cgp::prelude::*;
use reqwest::Client;
use sqlx::SqlitePool;

#[derive(CgpData)]
pub struct SqliteAndHttpClient {
    pub sqlite_pool: SqlitePool,

    pub http_client: Client,
}

#[cgp_impl(new BuildDefaultSqliteAndHttpClient)]
#[uses(CanRaiseError<sqlx::Error>)]
#[use_type(HasErrorType.Error)]
impl<Code, Input> Handler<Code, Input> {
    type Output = SqliteAndHttpClient;

    async fn handle(
        &self,
        _code: PhantomData<Code>,
        _input: Input,
        #[implicit] db_path: &str,
    ) -> Result<Self::Output, Error> {
        let sqlite_pool = SqlitePool::connect(db_path)
            .await
            .map_err(Self::raise_error)?;

        let http_client = Client::new();

        Ok(SqliteAndHttpClient {
            sqlite_pool,
            http_client,
        })
    }
}
