use core::str::FromStr;

use cgp::prelude::*;
use sqlx::SqlitePool;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};

#[cgp_auto_getter]
pub trait HasSqlitePath {
    fn db_path(&self) -> &str;
}

#[cgp_auto_getter]
pub trait HasSqliteOptions {
    fn db_options(&self) -> &str;
}

#[derive(HasField, HasFields, BuildField)]
pub struct SqliteClient {
    pub sqlite_pool: SqlitePool,
}

#[cgp_new_provider]
impl<Context, Code: Send, Input: Send> Handler<Context, Code, Input> for BuildSqliteClient
where
    Context: HasSqliteOptions + CanRaiseAsyncError<sqlx::Error>,
{
    type Output = SqliteClient;

    async fn handle(
        context: &Context,
        _code: PhantomData<Code>,
        _input: Input,
    ) -> Result<Self::Output, Context::Error> {
        let db_options = SqliteConnectOptions::from_str(context.db_options())
            .map_err(Context::raise_error)?
            .journal_mode(SqliteJournalMode::Wal);

        let sqlite_pool = SqlitePool::connect_with(db_options)
            .await
            .map_err(Context::raise_error)?;

        Ok(SqliteClient { sqlite_pool })
    }
}

#[cgp_new_provider]
impl<Context, Code: Send, Input: Send> Handler<Context, Code, Input> for BuildDefaultSqliteClient
where
    Context: HasSqlitePath + CanRaiseAsyncError<sqlx::Error>,
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
