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

    fn db_journal_mode(&self) -> &str;
}

#[derive(HasField, HasFields, BuildField)]
pub struct SqliteClient {
    pub sqlite_pool: SqlitePool,
}

#[cgp_new_provider]
impl<Build, Code: Send, Input: Send> Handler<Build, Code, Input> for BuildSqliteClient
where
    Build: HasSqliteOptions + CanRaiseError<sqlx::Error>,
{
    type Output = SqliteClient;

    async fn handle(
        build: &Build,
        _code: PhantomData<Code>,
        _input: Input,
    ) -> Result<Self::Output, Build::Error> {
        let journal_mode =
            SqliteJournalMode::from_str(build.db_journal_mode()).map_err(Build::raise_error)?;

        let db_options = SqliteConnectOptions::from_str(build.db_options())
            .map_err(Build::raise_error)?
            .journal_mode(journal_mode);

        let sqlite_pool = SqlitePool::connect_with(db_options)
            .await
            .map_err(Build::raise_error)?;

        Ok(SqliteClient { sqlite_pool })
    }
}

#[cgp_new_provider]
impl<Build, Code: Send, Input: Send> Handler<Build, Code, Input> for BuildDefaultSqliteClient
where
    Build: HasSqlitePath + CanRaiseError<sqlx::Error>,
{
    type Output = SqliteClient;

    async fn handle(
        build: &Build,
        _code: PhantomData<Code>,
        _input: Input,
    ) -> Result<Self::Output, Build::Error> {
        let sqlite_pool = SqlitePool::connect(build.db_path())
            .await
            .map_err(Build::raise_error)?;

        Ok(SqliteClient { sqlite_pool })
    }
}
