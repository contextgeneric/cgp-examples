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

#[cgp_new_provider]
impl<Context, Code: Send, Input: Send> Handler<Context, Code, Input> for BuildHttpClient
where
    Context: HasHttpClientConfig + CanRaiseAsyncError<reqwest::Error>,
{
    type Output = HttpClient;

    async fn handle(
        context: &Context,
        _code: PhantomData<Code>,
        _input: Input,
    ) -> Result<Self::Output, Context::Error> {
        let http_client = Client::builder()
            .user_agent(context.http_user_agent())
            .connect_timeout(Duration::from_secs(5))
            .build()
            .map_err(Context::raise_error)?;

        Ok(HttpClient { http_client })
    }
}
