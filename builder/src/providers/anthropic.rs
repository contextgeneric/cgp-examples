use cgp::prelude::*;
use rig::agent::Agent;
use rig::client::CompletionClient;
use rig::providers::anthropic::completion::CompletionModel;
use rig::providers::anthropic::{self, ANTHROPIC_VERSION_LATEST, ClientBuilder};

#[derive(CgpData)]
pub struct AnthropicClient {
    pub anthropic_client: anthropic::Client,
    pub anthropic_agent: Agent<CompletionModel>,
}

#[cgp_impl(new BuildDefaultAnthropicClient)]
#[use_type(HasErrorType.Error)]
impl<Code, Input> Handler<Code, Input> {
    type Output = AnthropicClient;

    async fn handle(
        &self,
        _code: PhantomData<Code>,
        _input: Input,
        #[implicit] anthropic_key: &str,
        #[implicit] llm_preamble: &str,
    ) -> Result<Self::Output, Error> {
        let anthropic_client = ClientBuilder::new(anthropic_key)
            .anthropic_version(ANTHROPIC_VERSION_LATEST)
            .build();

        let anthropic_agent = anthropic_client
            .agent(anthropic::CLAUDE_3_7_SONNET)
            .preamble(llm_preamble)
            .build();

        Ok(AnthropicClient {
            anthropic_client,
            anthropic_agent,
        })
    }
}
