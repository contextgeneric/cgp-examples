use cgp::prelude::*;
use rig::agent::Agent;
use rig::client::{CompletionClient, ProviderClient};
use rig::providers::openai;

#[cgp_auto_getter]
pub trait HasOpenAiConfig {
    fn open_ai_key(&self) -> &str;

    fn open_ai_model(&self) -> &str;

    fn llm_preamble(&self) -> &str;
}

#[derive(HasField, HasFields, BuildField)]
pub struct OpenAiClient {
    pub open_ai_client: openai::Client,
    pub open_ai_agent: Agent<openai::CompletionModel>,
}

#[cgp_new_provider]
impl<Build, Code, Input> Handler<Build, Code, Input> for BuildOpenAiClient
where
    Build: HasOpenAiConfig + HasErrorType,
{
    type Output = OpenAiClient;

    async fn handle(
        build: &Build,
        _code: PhantomData<Code>,
        _input: Input,
    ) -> Result<Self::Output, Build::Error> {
        let open_ai_client = openai::Client::new(build.open_ai_key());
        let open_ai_agent = open_ai_client
            .agent(build.open_ai_model())
            .preamble(build.llm_preamble())
            .build();

        Ok(OpenAiClient {
            open_ai_client,
            open_ai_agent,
        })
    }
}

#[cgp_new_provider]
impl<Build, Code, Input> Handler<Build, Code, Input> for BuildDefaultOpenAiClient
where
    Build: HasErrorType,
{
    type Output = OpenAiClient;

    async fn handle(
        _build: &Build,
        _code: PhantomData<Code>,
        _input: Input,
    ) -> Result<Self::Output, Build::Error> {
        let open_ai_client = openai::Client::from_env();
        let open_ai_agent = open_ai_client.agent("gpt-4o").build();

        Ok(OpenAiClient {
            open_ai_client,
            open_ai_agent,
        })
    }
}
