use std::env::{self, VarError};

use cgp::prelude::*;
use rig::agent::Agent;
use rig::client::CompletionClient;
use rig::providers::openai;

#[derive(CgpData)]
pub struct OpenAiClient {
    pub open_ai_client: openai::Client,
    pub open_ai_agent: Agent<openai::CompletionModel>,
}

#[cgp_impl(new BuildOpenAiClient)]
#[use_type(HasErrorType.Error)]
impl<Code, Input> Handler<Code, Input> {
    type Output = OpenAiClient;

    async fn handle(
        &self,
        _code: PhantomData<Code>,
        _input: Input,
        #[implicit] open_ai_key: &str,
        #[implicit] open_ai_model: &str,
        #[implicit] llm_preamble: &str,
    ) -> Result<Self::Output, Error> {
        let open_ai_client = openai::Client::new(open_ai_key);
        let open_ai_agent = open_ai_client
            .agent(open_ai_model)
            .preamble(llm_preamble)
            .build();

        Ok(OpenAiClient {
            open_ai_client,
            open_ai_agent,
        })
    }
}

#[cgp_impl(new BuildDefaultOpenAiClient)]
#[uses(CanRaiseError<VarError>)]
#[use_type(HasErrorType.Error)]
impl<Code, Input> Handler<Code, Input> {
    type Output = OpenAiClient;

    async fn handle(&self, _code: PhantomData<Code>, _input: Input) -> Result<Self::Output, Error> {
        let open_ai_key = env::var("OPENAI_API_KEY").map_err(Self::raise_error)?;
        let open_ai_client = openai::Client::new(&open_ai_key);
        let open_ai_agent = open_ai_client.agent("gpt-4o").build();

        Ok(OpenAiClient {
            open_ai_client,
            open_ai_agent,
        })
    }
}
