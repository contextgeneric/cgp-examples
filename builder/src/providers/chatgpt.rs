use cgp::prelude::*;
use rig::client::CompletionClient;
use rig::providers::openai;

#[cgp_auto_getter]
pub trait HasOpenAiConfig {
    fn open_ai_key(&self) -> &str;

    fn open_ai_model(&self) -> &str;
}

pub struct ChatGptClient {
    pub client: openai::Client,
    pub model: openai::CompletionModel,
}

#[cgp_new_provider]
impl<Context, Code: Send, Input: Send> Handler<Context, Code, Input> for BuildChatGptClient
where
    Context: HasOpenAiConfig + HasAsyncErrorType,
{
    type Output = ChatGptClient;

    async fn handle(
        context: &Context,
        _code: PhantomData<Code>,
        _input: Input,
    ) -> Result<Self::Output, Context::Error> {
        let client = openai::Client::new(context.open_ai_key());

        let model = client.completion_model(context.open_ai_model());

        Ok(ChatGptClient { client, model })
    }
}
