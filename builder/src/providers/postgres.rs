use cgp::prelude::*;
use sqlx::PgPool;

#[cgp_auto_getter]
pub trait HasPostgresUrl {
    fn postgres_url(&self) -> &str;
}

#[derive(HasField, HasFields, BuildField)]
pub struct PostgresClient {
    pub postgres_pool: PgPool,
}

#[cgp_new_provider]
impl<Build, Code, Input> Handler<Build, Code, Input> for BuildPostgresClient
where
    Build: HasPostgresUrl + CanRaiseError<sqlx::Error>,
{
    type Output = PostgresClient;

    async fn handle(
        build: &Build,
        _code: PhantomData<Code>,
        _input: Input,
    ) -> Result<Self::Output, Build::Error> {
        let postgres_pool = PgPool::connect(build.postgres_url())
            .await
            .map_err(Build::raise_error)?;

        Ok(PostgresClient { postgres_pool })
    }
}
