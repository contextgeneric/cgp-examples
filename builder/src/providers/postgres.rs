use cgp::prelude::*;
use sqlx::PgPool;

#[derive(CgpData)]
pub struct PostgresClient {
    pub postgres_pool: PgPool,
}

#[cgp_impl(new BuildPostgresClient)]
#[uses(CanRaiseError<sqlx::Error>)]
#[use_type(HasErrorType.Error)]
impl<Code, Input> Handler<Code, Input> {
    type Output = PostgresClient;

    async fn handle(
        &self,
        _code: PhantomData<Code>,
        _input: Input,
        #[implicit] postgres_url: &str,
    ) -> Result<Self::Output, Error> {
        let postgres_pool = PgPool::connect(postgres_url)
            .await
            .map_err(Self::raise_error)?;

        Ok(PostgresClient { postgres_pool })
    }
}
