use core::time::Duration;

use cgp::prelude::*;
use reqwest::Client;

#[cgp_auto_getter]
pub trait HasHttpClientConfig {
    fn http_user_agent(&self) -> &str;
}

#[derive(HasField, HasFields, BuildField)]
pub struct HttpClient {
    pub http_client: Client,
}

#[cgp_impl(new BuildHttpClient)]
impl<Build, Code, Input> Handler<Code, Input> for Build
where
    Build: HasHttpClientConfig + CanRaiseError<reqwest::Error>,
{
    type Output = HttpClient;

    async fn handle(
        build: &Build,
        _code: PhantomData<Code>,
        _input: Input,
    ) -> Result<Self::Output, Build::Error> {
        let http_client = Client::builder()
            .user_agent(build.http_user_agent())
            .connect_timeout(Duration::from_secs(5))
            .build()
            .map_err(Build::raise_error)?;

        Ok(HttpClient { http_client })
    }
}

#[cgp_impl(new BuildDefaultHttpClient)]
impl<Build, Code, Input> Handler<Code, Input> for Build
where
    Build: HasErrorType,
{
    type Output = HttpClient;

    async fn handle(
        _build: &Build,
        _code: PhantomData<Code>,
        _input: Input,
    ) -> Result<Self::Output, Build::Error> {
        let http_client = Client::new();
        Ok(HttpClient { http_client })
    }
}
