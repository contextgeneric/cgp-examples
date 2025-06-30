use cgp::prelude::*;
use rig::agent::Agent;
use rig::client::CompletionClient;
use rig::providers::anthropic::completion::CompletionModel;
use rig::providers::anthropic::{self, ANTHROPIC_VERSION_LATEST, ClientBuilder};

#[cgp_auto_getter]
pub trait HasAnthropicConfig {
    fn anthropic_key(&self) -> &str;

    fn llm_preamble(&self) -> &str;
}

#[derive(HasField, HasFields, BuildField)]
pub struct AnthropicClient {
    pub anthropic_client: anthropic::Client,
    pub anthropic_agent: Agent<CompletionModel>,
}

#[cgp_new_provider]
impl<Build, Code: Send, Input: Send> Handler<Build, Code, Input> for BuildDefaultAnthropicClient
where
    Build: HasAnthropicConfig + HasAsyncErrorType,
{
    type Output = AnthropicClient;

    async fn handle(
        build: &Build,
        _code: PhantomData<Code>,
        _input: Input,
    ) -> Result<Self::Output, Build::Error> {
        let anthropic_client = ClientBuilder::new(build.anthropic_key())
            .anthropic_version(ANTHROPIC_VERSION_LATEST)
            .build();

        let anthropic_agent = anthropic_client
            .agent(anthropic::CLAUDE_3_7_SONNET)
            .preamble(build.llm_preamble())
            .build();

        Ok(AnthropicClient {
            anthropic_client,
            anthropic_agent,
        })
    }
}
