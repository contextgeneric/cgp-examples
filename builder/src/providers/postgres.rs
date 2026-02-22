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

#[cgp_impl(new BuildPostgresClient)]
impl<Code, Input> Handler<Code, Input>
where
    Self: HasPostgresUrl + CanRaiseError<sqlx::Error>,
{
    type Output = PostgresClient;

    async fn handle(
        &self,
        _code: PhantomData<Code>,
        _input: Input,
    ) -> Result<Self::Output, Self::Error> {
        let postgres_pool = PgPool::connect(self.postgres_url())
            .await
            .map_err(Self::raise_error)?;

        Ok(PostgresClient { postgres_pool })
    }
}
