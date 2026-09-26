use core::str::FromStr;

use cgp::prelude::*;
use sqlx::SqlitePool;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};

#[derive(CgpData)]
pub struct SqliteClient {
    pub sqlite_pool: SqlitePool,
}

#[cgp_impl(new BuildSqliteClient)]
#[uses(CanRaiseError<sqlx::Error>)]
#[use_type(HasErrorType.Error)]
impl<Code, Input> Handler<Code, Input> {
    type Output = SqliteClient;

    async fn handle(
        &self,
        _code: PhantomData<Code>,
        _input: Input,
        #[implicit] db_options: &str,
        #[implicit] db_journal_mode: &str,
    ) -> Result<Self::Output, Error> {
        let journal_mode =
            SqliteJournalMode::from_str(db_journal_mode).map_err(Self::raise_error)?;

        let db_options = SqliteConnectOptions::from_str(db_options)
            .map_err(Self::raise_error)?
            .journal_mode(journal_mode);

        let sqlite_pool = SqlitePool::connect_with(db_options)
            .await
            .map_err(Self::raise_error)?;

        Ok(SqliteClient { sqlite_pool })
    }
}

#[cgp_impl(new BuildDefaultSqliteClient)]
#[uses(CanRaiseError<sqlx::Error>)]
#[use_type(HasErrorType.Error)]
impl<Code, Input> Handler<Code, Input> {
    type Output = SqliteClient;

    async fn handle(
        &self,
        _code: PhantomData<Code>,
        _input: Input,
        #[implicit] db_path: &str,
    ) -> Result<Self::Output, Error> {
        let sqlite_pool = SqlitePool::connect(db_path)
            .await
            .map_err(Self::raise_error)?;

        Ok(SqliteClient { sqlite_pool })
    }
}
