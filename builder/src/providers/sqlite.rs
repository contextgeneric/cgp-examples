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

#[cgp_impl(new BuildSqliteClient)]
impl<Code, Input> Handler<Code, Input>
where
    Self: HasSqliteOptions + CanRaiseError<sqlx::Error>,
{
    type Output = SqliteClient;

    async fn handle(
        &self,
        _code: PhantomData<Code>,
        _input: Input,
    ) -> Result<Self::Output, Self::Error> {
        let journal_mode =
            SqliteJournalMode::from_str(self.db_journal_mode()).map_err(Self::raise_error)?;

        let db_options = SqliteConnectOptions::from_str(self.db_options())
            .map_err(Self::raise_error)?
            .journal_mode(journal_mode);

        let sqlite_pool = SqlitePool::connect_with(db_options)
            .await
            .map_err(Self::raise_error)?;

        Ok(SqliteClient { sqlite_pool })
    }
}

#[cgp_impl(new BuildDefaultSqliteClient)]
impl<Code, Input> Handler<Code, Input>
where
    Self: HasSqlitePath + CanRaiseError<sqlx::Error>,
{
    type Output = SqliteClient;

    async fn handle(
        &self,
        _code: PhantomData<Code>,
        _input: Input,
    ) -> Result<Self::Output, Self::Error> {
        let sqlite_pool = SqlitePool::connect(self.db_path())
            .await
            .map_err(Self::raise_error)?;

        Ok(SqliteClient { sqlite_pool })
    }
}
