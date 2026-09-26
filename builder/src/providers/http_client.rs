use core::time::Duration;

use cgp::prelude::*;
use reqwest::Client;

#[derive(CgpData)]
pub struct HttpClient {
    pub http_client: Client,
}

#[cgp_impl(new BuildHttpClient)]
#[uses(CanRaiseError<reqwest::Error>)]
#[use_type(HasErrorType.Error)]
impl<Code, Input> Handler<Code, Input> {
    type Output = HttpClient;

    async fn handle(
        &self,
        _code: PhantomData<Code>,
        _input: Input,
        #[implicit] http_user_agent: &str,
    ) -> Result<Self::Output, Error> {
        let http_client = Client::builder()
            .user_agent(http_user_agent)
            .connect_timeout(Duration::from_secs(5))
            .build()
            .map_err(Self::raise_error)?;

        Ok(HttpClient { http_client })
    }
}

#[cgp_impl(new BuildDefaultHttpClient)]
#[use_type(HasErrorType.Error)]
impl<Code, Input> Handler<Code, Input> {
    type Output = HttpClient;

    async fn handle(&self, _code: PhantomData<Code>, _input: Input) -> Result<Self::Output, Error> {
        let http_client = Client::new();
        Ok(HttpClient { http_client })
    }
}
