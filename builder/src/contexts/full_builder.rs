use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::core::field::impls::CanBuildFrom;
use cgp::extra::dispatch::BuildAndMergeOutputs;
use cgp::extra::handler::CanHandle;
use cgp::prelude::*;
use cgp_error_anyhow::{Error, RaiseAnyhowError, UseAnyhowError};
use serde::Deserialize;

use crate::contexts::app::App;
use crate::providers::{BuildHttpClient, BuildOpenAiClient, BuildSqliteClient};

#[derive(HasField, Deserialize)]
pub struct FullAppBuilder {
    pub db_options: String,
    pub db_journal_mode: String,
    pub http_user_agent: String,
    pub open_ai_key: String,
    pub open_ai_model: String,
    pub llm_preamble: String,
}

delegate_components! {
    FullAppBuilder {
        ErrorTypeProviderComponent:
            UseAnyhowError,
        ErrorRaiserComponent:
            RaiseAnyhowError,
        HandlerComponent:
            BuildAndMergeOutputs<
                App,
                Product![
                    BuildSqliteClient,
                    BuildHttpClient,
                    BuildOpenAiClient,
                ]>,
    }
}

check_components! {
    CanUseFullAppBuilder for FullAppBuilder {
        HandlerComponent: ((), ()),
    }
}

#[cgp_new_provider]
impl<Code: Send, Input: Send> Handler<FullAppBuilder, Code, Input> for BuildApp {
    type Output = App;

    async fn handle(
        context: &FullAppBuilder,
        code: PhantomData<Code>,
        _input: Input,
    ) -> Result<App, Error> {
        let app = App::builder()
            .build_from(BuildSqliteClient::handle(context, code, ()).await?)
            .build_from(BuildHttpClient::handle(context, code, ()).await?)
            .build_from(BuildOpenAiClient::handle(context, code, ()).await?)
            .finalize_build();

        Ok(app)
    }
}

pub async fn main() -> Result<(), Error> {
    let builder = FullAppBuilder {
        db_options: "file:./db.sqlite".to_owned(),
        db_journal_mode: "WAL".to_owned(),
        http_user_agent: "SUPER_AI_AGENT".to_owned(),
        open_ai_key: "1234567890".to_owned(),
        open_ai_model: "gpt-4o".to_owned(),
        llm_preamble: "You are a helpful assistant".to_owned(),
    };

    let _app = builder.handle(PhantomData::<()>, ()).await?;

    /* Call methods on the app here */

    Ok(())
}
