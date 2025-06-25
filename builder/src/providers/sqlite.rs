use cgp::prelude::*;
use sqlx::SqlitePool;

#[cgp_auto_getter]
pub trait HasSqliteConfig {
    fn db_path(&self) -> &str;
}

#[derive(HasField, HasFields, BuildField)]
pub struct SqliteClient {
    pub sqlite_pool: SqlitePool,
}

#[cgp_new_provider]
impl<Context, Code: Send, Input: Send> Handler<Context, Code, Input> for BuildSqliteClient
where
    Context: HasSqliteConfig + CanRaiseAsyncError<sqlx::Error>,
{
    type Output = SqliteClient;

    async fn handle(
        context: &Context,
        _code: PhantomData<Code>,
        _input: Input,
    ) -> Result<Self::Output, Context::Error> {
        let sqlite_pool = SqlitePool::connect(context.db_path())
            .await
            .map_err(Context::raise_error)?;

        Ok(SqliteClient { sqlite_pool })
    }
}
