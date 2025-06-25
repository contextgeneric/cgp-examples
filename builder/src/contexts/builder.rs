use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::extra::dispatch::{BuildWithHandlers, HandleAndBuild};
use cgp::prelude::*;
use cgp_error_anyhow::{RaiseAnyhowError, UseAnyhowError};

use crate::contexts::App;
use crate::providers::{BuildOpenAiClient, BuildSqliteClient};

#[cgp_context]
#[derive(HasField)]
pub struct AppBuilder {
    pub open_ai_key: String,
    pub open_ai_model: String,
    pub db_path: String,
}

delegate_components! {
    AppBuilderComponents {
        ErrorTypeProviderComponent:
            UseAnyhowError,
        ErrorRaiserComponent:
            RaiseAnyhowError,
        HandlerComponent:
            BuildWithHandlers<
                App,
                Product![
                    HandleAndBuild<BuildOpenAiClient>,
                    HandleAndBuild<BuildSqliteClient>,
                ]>,
    }
}

check_components! {
    CanUseAppBuilder for AppBuilder {
        HandlerComponent: ((), ()),
    }
}
