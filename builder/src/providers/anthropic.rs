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

#[cgp_impl(new BuildDefaultAnthropicClient)]
impl<Code, Input> Handler<Code, Input>
where
    Self: HasAnthropicConfig + HasErrorType,
{
    type Output = AnthropicClient;

    async fn handle(
        &self,
        _code: PhantomData<Code>,
        _input: Input,
    ) -> Result<Self::Output, Self::Error> {
        let anthropic_client = ClientBuilder::new(self.anthropic_key())
            .anthropic_version(ANTHROPIC_VERSION_LATEST)
            .build();

        let anthropic_agent = anthropic_client
            .agent(anthropic::CLAUDE_3_7_SONNET)
            .preamble(self.llm_preamble())
            .build();

        Ok(AnthropicClient {
            anthropic_client,
            anthropic_agent,
        })
    }
}
