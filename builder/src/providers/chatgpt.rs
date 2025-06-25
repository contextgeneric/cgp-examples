use cgp::prelude::*;
use rig::client::CompletionClient;
use rig::providers::openai;

#[cgp_auto_getter]
pub trait HasOpenAiConfig {
    fn open_ai_key(&self) -> &str;

    fn open_ai_model(&self) -> &str;
}

#[derive(HasField, HasFields, BuildField)]
pub struct OpenAiClient {
    pub open_ai_client: openai::Client,
    pub open_ai_model: openai::CompletionModel,
}

#[cgp_new_provider]
impl<Context, Code: Send, Input: Send> Handler<Context, Code, Input> for BuildOpenAiClient
where
    Context: HasOpenAiConfig + HasAsyncErrorType,
{
    type Output = OpenAiClient;

    async fn handle(
        context: &Context,
        _code: PhantomData<Code>,
        _input: Input,
    ) -> Result<Self::Output, Context::Error> {
        let open_ai_client = openai::Client::new(context.open_ai_key());
        let open_ai_model = open_ai_client.completion_model(context.open_ai_model());

        Ok(OpenAiClient {
            open_ai_client,
            open_ai_model,
        })
    }
}
