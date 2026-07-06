use cgp::prelude::*;

// The core capability every endpoint is a case of. `#[cgp_component]` splits it into the
// consumer trait `CanHandleApi` (what callers use) and a provider trait `ApiHandler<Context>`
// (what handlers implement). It is generic over an `Api` marker, so a context can wire a
// different handler per endpoint; `Request`/`Response` let each endpoint fix its own shapes.
// `#[async_trait]` makes the async method lint-clean, and `#[use_type(HasErrorType.Error)]`
// imports the context's abstract error type so the signature can name it as bare `Error`.
#[cgp_component(ApiHandler)]
#[prefix(@app.api in DefaultNamespace)]
#[async_trait]
#[use_type(HasErrorType.Error)]
pub trait CanHandleApi<Api> {
    type Request;

    type Response;

    async fn handle_api(
        &self,
        _api: PhantomData<Api>,
        request: Self::Request,
    ) -> Result<Self::Response, Error>;
}

// A `Send`-carrying view of `CanHandleApi`, needed because the async trait method drops the
// `Send` bound its future would need to be spawned on a multi-threaded runtime. This is a
// plain (non-CGP) trait whose method spells out `+ Send`; it is implemented per concrete
// context in `contexts/app.rs`, where the compiler can prove the bound. See the README.
pub trait CanHandleApiSend<Api>:
    CanHandleApi<Api, Request: Send, Response: Send> + Send + Sync
{
    fn handle_api_send(
        &self,
        _api: PhantomData<Api>,
        request: Self::Request,
    ) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send;
}

/// Endpoint marker for `POST /transfer`, used only as a type-level tag on `CanHandleApi`.
pub struct TransferApi;

/// Endpoint marker for `GET /balance`, used only as a type-level tag on `CanHandleApi`.
pub struct QueryBalanceApi;
