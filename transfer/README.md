# Money-Transfer API — a Context-Generic Programming walkthrough

This example is a small money-transfer web service: it lets a user check an account
balance and move funds to another user, served over HTTP. Its real purpose is to teach
**Context-Generic Programming (CGP)** — a modular style of Rust in which a capability is
declared once, implemented many times, and each implementation is chosen per application
by *wiring* rather than by hardcoding. The service is deliberately tiny so that the CGP
techniques, not the business logic, are what you take away.

The walkthrough assumes you know Rust — traits, generics, `async`/`await` — but nothing
about CGP. Every CGP construct is explained where it first appears, so you can read this
file top to bottom without any other reference. If you only skim, the first sentence of
each section carries the point.

## What the service does

The server exposes two endpoints, both authenticated with HTTP Basic auth. `GET /balance`
returns the caller's balance in a given currency, and `POST /transfer` moves a quantity of
one currency from the caller to a named recipient. Failures come back as HTTP status codes
— `401` when you are not logged in, `400` for a bad request such as transferring to
yourself, `404` when a user is unknown.

The [example.sh](example.sh) script shows the shape of a session against a running server:

```bash
export ALICE_AUTH="Authorization: Basic $(echo -n 'alice:wonderland' | base64)"

# Alice checks her EUR balance
curl -i -H "$ALICE_AUTH" 'http://localhost:8080/balance?currency=EUR'

# Alice transfers 10 EUR to Bob
curl -i -XPOST -H "$ALICE_AUTH" 'http://localhost:8080/transfer?currency=EUR&recipient=bob&quantity=10'
```

The data lives in memory, seeded with two users (Alice and Bob) in
[src/contexts/app.rs](src/contexts/app.rs). A real deployment would swap the in-memory
store for a database, and — as the last section shows — that swap touches none of the
request-handling code.

## How to run it

The example is an ordinary Cargo binary. Start the server and then drive it with the
script above:

```bash
cargo run --bin server        # listens on 0.0.0.0:8080
./example.sh                  # in another terminal
```

## A five-minute CGP primer

Read this section once and the rest of the walkthrough falls into place. It states the
whole mental model in terms this example uses.

**CGP exists to get around Rust's coherence rules.** Rust allows at most one implementation
of a trait for a given type, and forbids implementing a foreign trait for a foreign type.
That makes it hard to offer several interchangeable implementations of one interface, or to
let the final application choose an implementation for a type a library defined. CGP
sidesteps both limits, so the same `handle_api` interface can have a mock backend today and
a database backend tomorrow, chosen without editing the interface or its callers.

**A CGP capability is split into two traits: a consumer trait you call, and a provider
trait you implement.** When you write `#[cgp_component(MoneyTransferrer)]` on a trait
`CanTransferMoney`, the macro keeps `CanTransferMoney` as the **consumer trait** — the
one callers use, with the familiar `&self` receiver — and generates a matching **provider
trait** `MoneyTransferrer<Context>`, where the original `Self` has moved into an explicit
`Context` type parameter. The consumer reads as a verb (`CanTransferMoney`), the provider
as a noun (`MoneyTransferrer`). You call the consumer; you implement the provider.

**A provider is a zero-sized marker type that implements a provider trait.** `UseMockedApp`
and `NoTransferToSelf` in this example are providers: structs with no fields, existing only
as names at the type level. Because a provider implements `MoneyTransferrer<Context>` for
*its own* struct over a generic `Context` — rather than implementing a trait for the context
— Rust's coherence rules never block it, and you can define as many alternative providers
for one capability as you like. One consequence trips up every newcomer: inside a provider,
`self` and `Self` refer to the **context**, never to the provider struct. The provider
struct holds no data and is never constructed.

**Wiring is a per-context, compile-time table that says which provider implements each
capability.** The `delegate_components!` macro builds it. An entry like
`MoneyTransferrerComponent: UseMockedApp` means "for this context, the money-transfer
capability is supplied by `UseMockedApp`." The table is pure trait resolution — there is no
runtime lookup, no dynamic dispatch, and no vtable. The compiler picks the provider during
type-checking and compiles the call down to a direct, static function call, so all of this
machinery costs nothing at runtime.

**A provider declares the dependencies it needs in a `where` clause, and those dependencies
stay hidden from callers.** This is CGP's form of dependency injection, and it is the single
most important idea. `UseBasicAuth` needs the ability to look up a hashed password and check
it; it says so with `#[uses(CanQueryUserHashedPassword, CanCheckPassword)]`, which adds those
as bounds on its own implementation. A caller that uses the authenticated handler never sees
those requirements — they are satisfied one level down, by whatever the context wires for
them. This is why endpoints can be written against abstract capabilities and stay ignorant
of the concrete backend behind them.

**Wiring is checked with `check_components!`.** Because wiring is resolved lazily, a missing
dependency would otherwise only surface far away, at the point of use, as a confusing error.
`check_components!` asserts at compile time that a context is fully and correctly wired,
reporting the exact missing piece at the wiring site. A green build *is* the passing test.

With those six ideas — two traits, providers, wiring, impl-side dependencies, and checking —
you can read every construct below.

## Map of the code

The crate is organized so that the *interface* of the service is separated from every
*implementation* of it, which is the separation CGP is built to exploit. Five directories
under [src/](src) each hold one kind of thing:

- [src/interfaces/](src/interfaces) — the CGP components: the consumer traits and abstract
  types that define *what* the service can do, naming no concrete type or backend.
- [src/providers/](src/providers) — the providers that implement those components: the
  endpoint handlers, the reusable wrappers, the in-memory backend, and the error mapping.
- [src/types/](src/types) — the concrete Rust types a deployment plugs in: the currency
  enum, the error struct, and the request structs.
- [src/namespaces/](src/namespaces) — reusable bundles of wiring that a context inherits,
  so the application's own wiring stays short.
- [src/contexts/](src/contexts) — the concrete application type, `MockApp`, that ties
  everything together by wiring each component to a provider.

The [bin/server.rs](bin/server.rs) entry point builds a `MockApp`, mounts its endpoints on
an Axum router, and serves them. The sections below follow the data inward: from the
abstract vocabulary, through the handlers and backend, to the wiring and the HTTP layer.

## 1. Abstract domain types

The service never names a concrete user id, currency, or amount — it names *abstract types*
that a context supplies later. An abstract type in CGP is a trait carrying a single
associated type, and the `#[cgp_type]` macro is the shorthand for defining one. In
[src/interfaces/types.rs](src/interfaces/types.rs):

```rust
#[cgp_type]
#[prefix(@app.finance.types in DefaultNamespace)]
pub trait HasCurrencyType {
    type Currency: Display;
}
```

`HasCurrencyType` says "a context has *some* currency type, and whatever it is, it can be
displayed." Generic code refers to `Self::Currency` without committing to `String`, an enum,
or anything else; the concrete choice is made once, at wiring time. This is what lets a
single balance-query handler serve one deployment whose currency is a rich enum and another
whose currency is a bare string, with no change to the handler. The example defines five
such types — user id, quantity, currency, password, and hashed password — each carrying only
the bound the rest of the code actually needs.

The `#[prefix(...)]` attribute files each component under a dotted path in a shared
namespace. Ignore it for now; it is a wiring-organization detail explained in
[section 7](#7-assembling-the-app-with-namespaces).

A context supplies a concrete type by wiring the component to `UseType<T>`, a built-in
provider that means "the abstract type is `T`." `MockApp` picks its currency with
`CurrencyTypeProviderComponent: UseType<DemoCurrency>`, and the `Display` bound is checked
against `DemoCurrency` right there at the wiring site.

## 2. Status-coded errors

Endpoints fail with an HTTP status code, but the handlers never build one directly — they
raise through an application-specific error capability, so the mapping from a domain failure
to a status code lives in exactly one place. That capability, in
[src/interfaces/error.rs](src/interfaces/error.rs), is a component:

```rust
#[cgp_component(HttpErrorRaiser)]
#[prefix(@app.error in DefaultNamespace)]
#[use_type(HasErrorType.Error)]
pub trait CanRaiseHttpError<Code, Detail> {
    fn raise_http_error(_code: Code, detail: Detail) -> Error;
}
```

Two CGP features appear here. `CanRaiseHttpError` is generic over a `Code` marker (one of
the zero-sized `ErrUnauthorized`, `ErrBadRequest`, `ErrNotFound` structs) and a `Detail`
value, so `Self::raise_http_error(ErrNotFound, "user not found")` names the status class at
the type level. And `#[use_type(HasErrorType.Error)]` imports the context's *abstract error
type*: `HasErrorType` is CGP's built-in "a context has one shared error type `Error`"
component, and the attribute both makes it a supertrait and rewrites the bare `Error` in the
signature to the fully-qualified associated type. Without the attribute the method would have
to be written `-> <Self as HasErrorType>::Error`; the attribute lets it read `-> Error`.

The status codes come from an ordinary trait that turns each marker into a code, so a provider
can map `ErrUnauthorized` to `401` with no match statement:

```rust
pub trait IsStatusCode {
    fn status_code() -> StatusCode;
}
impl IsStatusCode for ErrUnauthorized { /* UNAUTHORIZED */ }
```

The provider that satisfies the capability, `DisplayHttpError` in
[src/providers/error.rs](src/providers/error.rs), is generic over both the code and the
detail, so one provider serves every `raise_http_error` call whose detail is `Display`:

```rust
#[cgp_impl(new DisplayHttpError)]
#[use_type(HasErrorType.{Error = AppError})]
impl<Code, Detail> HttpErrorRaiser<Code, Detail>
where
    Code: IsStatusCode,
    Detail: Display,
{
    fn raise_http_error(_code: Code, detail: Detail) -> AppError { /* ... */ }
}
```

`#[cgp_impl(new DisplayHttpError)]` is the idiomatic way to write a provider: you write the
impl in consumer-trait style, and the `new` keyword also declares the `struct DisplayHttpError;`
for you. The `#[use_type(HasErrorType.{Error = AppError})]` equality form goes one step
further than before — it *pins* the abstract error to the concrete `AppError`, which is why
the method body can construct an `AppError` value directly. A sibling provider,
`HandleHttpErrorWithAnyhow`, handles details that are already an `anyhow::Error`; the wiring
picks between the two per detail type.

## 3. The API-handler component

Every endpoint is one case of a single component that dispatches on a marker type naming the
API. In [src/interfaces/api.rs](src/interfaces/api.rs):

```rust
#[cgp_component(ApiHandler)]
#[prefix(@app.api in DefaultNamespace)]
#[async_trait]
#[use_type(HasErrorType.Error)]
pub trait CanHandleApi<Api> {
    type Request;
    type Response;

    async fn handle_api(&self, _api: PhantomData<Api>, request: Self::Request)
        -> Result<Self::Response, Error>;
}

pub struct TransferApi;
pub struct QueryBalanceApi;
```

Because the component is generic over `Api`, a context can bind a *different* provider for
each endpoint marker — `TransferApi` to one, `QueryBalanceApi` to another — while the
`Request` and `Response` associated types let each endpoint fix its own input and output
shape. The `PhantomData<Api>` argument carries the endpoint choice at the type level, so the
dispatch is resolved at compile time with no runtime branch. `#[async_trait]` lets the method
be written as a natural `async fn` while generating the lint-clean `-> impl Future`
declaration underneath.

## 4. The endpoint handlers

Each endpoint is a provider for `ApiHandler` that depends on business *capabilities* rather
than on any concrete backend. The transfer endpoint, in
[src/providers/api_handlers/transfer.rs](src/providers/api_handlers/transfer.rs), reads the
logged-in sender and the transfer details from its request, then calls `CanTransferMoney` —
itself an abstract async capability the context implements however it likes:

```rust
#[cgp_impl(new HandleTransfer<Request>)]
#[uses(CanTransferMoney, CanRaiseHttpError<ErrUnauthorized, String>)]
#[use_type(HasErrorType.Error)]
impl<Api, Request> ApiHandler<Api>
where
    Request: HasLoggedInUser<Self> + HasTransferMoneyFields<Self>,
{
    type Request = Request;
    type Response = ();

    async fn handle_api(&self, _api: PhantomData<Api>, request: Request) -> Result<(), Error> {
        let sender = request.logged_in_user().as_ref().ok_or_else(|| {
            Self::raise_http_error(ErrUnauthorized, "you must first login".into())
        })?;
        self.transfer_money(sender, request.recipient(), request.currency(), request.quantity())
            .await?;
        Ok(())
    }
}
```

The handler is generic over its request shape: `HandleTransfer<Request>` works for any
`Request` that exposes a logged-in user and the transfer fields through the getter traits
named in its `where` clause, so the same logic serves whatever request struct a deployment
decodes. The `#[uses(...)]` attribute is where the impl-side dependencies live — it declares
that the context must provide `CanTransferMoney` and the ability to raise an unauthorized
error, and those requirements never leak into the `ApiHandler` interface. Inside the body,
`self.transfer_money(...)` and `Self::raise_http_error(...)` call those wired capabilities;
which provider actually runs is decided by the context's wiring.

The getter traits in the `where` clause — `HasLoggedInUser`, `HasTransferMoneyFields` — are
defined with `#[cgp_auto_getter]`, which turns a trait of `&self` accessors into a blanket
implementation that reads same-named fields off the context. They read fields from the
*request* type (`Request: HasTransferMoneyFields<Self>`), which is why they are getter traits
rather than the simpler implicit arguments used elsewhere: an implicit argument can only read
a field from `self`, and here the fields live on a separate request value.

The balance query in
[src/providers/api_handlers/query_balance.rs](src/providers/api_handlers/query_balance.rs)
has the same shape but returns a value, so it fixes a `Response` type. Its
`QueryBalanceResponse<App>` stays generic over the context's abstract `Quantity`, and
`#[derive(Serialize)]` lets the JSON layer encode it.

## 5. Reusable wrappers as higher-order providers

Cross-cutting concerns — decoding, authentication, JSON encoding — are handlers that wrap
another handler. A provider that takes another provider as a type parameter and delegates
part of its work to it is called a **higher-order provider**, and it is how CGP composes
behavior. Each wrapper implements `ApiHandler` itself, takes an inner handler as a generic
parameter, and threads the call through, transforming the request or response on the way.

`UseBasicAuth`, in
[src/providers/api_handlers/basic_auth.rs](src/providers/api_handlers/basic_auth.rs),
authenticates before delegating: it resolves a Basic-auth header into a logged-in user,
mutates the request in place, and then calls the inner handler.

```rust
#[cgp_impl(new UseBasicAuth<InHandler>)]
#[uses(CanQueryUserHashedPassword, CanCheckPassword)]
#[use_type(HasErrorType.Error)]
#[use_provider(InHandler: ApiHandler<Api>)]
impl<Api, InHandler> ApiHandler<Api>
where
    InHandler::Request: HasLoggedInUserMut<Self> + HasBasicAuthHeader<Self>,
    Self::UserId: Clone,
{
    // ... authenticate, then: InHandler::handle_api(self, api, request).await
}
```

The `#[use_provider(InHandler: ApiHandler<Api>)]` attribute is the key detail. A provider
trait moves `Self` into a leading context parameter, so the inner bound is really
`InHandler: ApiHandler<Self, Api>` — with the context slot filled in. Writing that by hand is
easy to get wrong, so `#[use_provider]` lets you write `InHandler: ApiHandler<Api>` and
supplies the context argument for you. Note the body calls the inner handler as an
associated function, `InHandler::handle_api(self, ...)`, passing the context explicitly —
not `self.handle_api(...)`, which would route back through the context's own wiring instead
of the named inner provider.

Two more wrappers complete the set. `HandleFromRequest` in
[src/providers/api_handlers/from_request.rs](src/providers/api_handlers/from_request.rs)
adapts the request type, letting an endpoint that wants a clean domain request sit behind a
handler whose request is the raw type the HTTP layer produces (it requires
`Request: Into<InHandler::Request>`). `ResponseToJson` in
[src/providers/axum/json.rs](src/providers/axum/json.rs) adapts in the other direction,
wrapping whatever the inner handler returns in an Axum `Json` envelope.

Because each wrapper is itself an `ApiHandler`, they nest into a pipeline. The balance
endpoint is wired to:

```rust
HandleFromRequest<AxumQueryBalanceRequest,
    ResponseToJson<UseBasicAuth<HandleQueryBalance<QueryBalanceRequest>>>>
```

which reads outside-in as the stages a request passes through: decode the raw Axum request,
JSON-encode the response, authenticate, run the endpoint. Each layer adds exactly one
concern, and the endpoint at the center is oblivious to all of them.

## 6. The in-memory backend

The business capabilities — querying a balance, transferring money, checking passwords — are
satisfied by a single provider, `UseMockedApp`, that reads its data from context fields. It
lives in [src/providers/mocked.rs](src/providers/mocked.rs) and implements each capability by
reaching into maps stored on the context, pulled in as `#[implicit]` arguments:

```rust
#[cgp_impl(UseMockedApp)]
#[default_impl(@app.finance.UserBalanceQuerierComponent in MockNamespace)]
#[uses(CanRaiseHttpError<ErrNotFound, String>)]
#[use_type(HasUserIdType.UserId, HasCurrencyType.Currency, HasQuantityType.Quantity, HasErrorType.Error)]
impl UserBalanceQuerier
where /* ... */
{
    async fn query_user_balance(
        &self,
        user: &UserId,
        currency: &Currency,
        #[implicit] user_balances: &Arc<Mutex<BTreeMap<(UserId, Currency), Quantity>>>,
    ) -> Result<Quantity, Error> { /* ... */ }
}
```

An `#[implicit]` argument is the preferred way to read a field from a provider's own context.
The macro removes the argument from the public signature, adds a hidden `where` bound saying
the context has a field of that name and type, and binds the value at the top of the body.
Here `user_balances` reads the same-named field off the context by shared reference, with no
clone and no getter trait to declare. The `#[use_type(...)]` attribute imports several
abstract types at once, so the signature can name `UserId`, `Currency`, and `Quantity`
without any `Self::` qualification.

Swapping this backend for a real one is a one-line wiring change. Because `UseMockedApp` is
selected per context, a `UsePostgres` provider that implements the same capabilities would
replace it without touching a single endpoint or wrapper — which is the entire point of the
consumer/provider split.

A business capability can be wrapped just like an API handler. `NoTransferToSelf` in
[src/providers/finance.rs](src/providers/finance.rs) is a higher-order provider for
`MoneyTransferrer` that rejects a self-transfer and otherwise delegates to an inner transfer
provider — the same wrapping pattern as the HTTP handlers, applied to a domain capability.

## 7. Assembling the app with namespaces

A concrete context becomes the running application by resolving every abstract type and every
component. Spelling all of that out on the context would be long and repetitive, so the
example lifts the shared wiring into reusable **namespaces**. A namespace is a named table of
wiring entries that a context inherits wholesale and can then selectively override — CGP's
form of preset configuration, resolved entirely at compile time.

Namespaces are populated from two sides. Components file themselves under a path with the
`#[prefix(@path in DefaultNamespace)]` attribute seen throughout the interfaces — this is
what those attributes were for. Providers register themselves as a namespace's default with
the `#[default_impl(@path in MockNamespace)]` attribute seen on `UseMockedApp` above, which
places the wiring next to the implementation it wires. `MockNamespace` in
[src/namespaces/mock.rs](src/namespaces/mock.rs) then only needs to spell out the entries
that have no `#[cgp_impl]` block to attach to — the concrete error type, the per-detail HTTP
error dispatch, and the abstract type choices:

```rust
cgp_namespace! {
    new MockNamespace: DefaultNamespace {
        @cgp.core.error.ErrorTypeProviderComponent: UseType<AppError>,
        @app.finance.types.QuantityTypeProviderComponent: UseType<u64>,
        @app.finance.types.CurrencyTypeProviderComponent: UseType<DemoCurrency>,
        // ... and the HTTP-error dispatch and the auth types
    }
}
```

The API surface — which handler pipeline serves each endpoint — is a second reusable table,
`DefaultApiHandlers` in [src/namespaces/api_handlers.rs](src/namespaces/api_handlers.rs),
keyed by the API marker so any backend can pull it in.

With the backend defaults in `MockNamespace` and the endpoints in `DefaultApiHandlers`, the
application's own wiring in [src/contexts/app.rs](src/contexts/app.rs) shrinks to three
decisions:

```rust
delegate_components! {
    MockApp {
        namespace MockNamespace;

        for <Key, Value> in DefaultApiHandlers {
            @app.api.ApiHandlerComponent.Key: Value,
        }

        @app.finance.MoneyTransferrerComponent: NoTransferToSelf<UseMockedApp>,
    }
}
```

Each line states a decision, not a mechanical fact. `namespace MockNamespace;` joins the
namespace, inheriting every default backend and type. The `for` loop pulls each entry of
`DefaultApiHandlers` onto the `ApiHandler` dispatch path, wiring both endpoints at once. And
the last line overrides the money-transfer capability to wrap the mock backend in
`NoTransferToSelf`, guarding against self-transfers. The override is possible only because
`MockNamespace` deliberately does *not* claim the money-transfer path itself — a context can
override a path the namespace routes *to*, but not one the namespace already terminates.

Because CGP wiring is checked lazily, a companion `check_components!` block proves at compile
time that every endpoint is fully satisfied, listing the API markers to verify for the
generic `ApiHandler` component:

```rust
check_components! {
    MockApp {
        QuantityTypeProviderComponent,
        UserBalanceQuerierComponent,
        MoneyTransferrerComponent,
        ApiHandlerComponent: [QueryBalanceApi, TransferApi],
    }
}
```

If a provider's transitive dependency were unmet — a missing field, an unwired type — this
block would fail to compile and name the exact gap, at the wiring site rather than at some
distant call.

## 8. Serving over HTTP

Handing the handlers to Axum needs one bound the component cannot express: that each
handler's future is `Send`. Axum runs on a multi-threaded, work-stealing runtime that may
move a suspended task between threads, so the futures it drives must be `Send`. But the
`async fn` in `CanHandleApi` desugars to a bare `impl Future` with no such bound, and stable
Rust has no way to require it generically through the trait.

The fix, in [src/contexts/app.rs](src/contexts/app.rs), is a plain trait whose method spells
out `+ Send` on its return type, implemented for the concrete context:

```rust
pub trait CanHandleApiSend<Api>:
    CanHandleApi<Api, Request: Send, Response: Send> + Send + Sync
{
    fn handle_api_send(&self, _api: PhantomData<Api>, request: Self::Request)
        -> impl Future<Output = Result<Self::Response, Self::Error>> + Send;
}

impl CanHandleApiSend<QueryBalanceApi> for MockApp {
    async fn handle_api_send(&self, api: PhantomData<QueryBalanceApi>, request: Self::Request)
        -> Result<Self::Response, Self::Error> {
        self.handle_api(api, request).await
    }
}
// ... and the same one-line forwarding impl for TransferApi
```

Each impl merely forwards to `handle_api`, but at a *concrete* context and API the awaited
future is a concrete type whose `Send`-ness the compiler can verify on its own — which is why
the impls cannot be folded into one generic blanket impl. The repetition is the price of a
missing language feature (Return Type Notation), not a CGP requirement.

The Axum routing layer in [src/providers/axum/routes.rs](src/providers/axum/routes.rs) then
bounds `App: CanHandleApiSend<Api>` and mounts each endpoint: `add_route` reads the request
out of the HTTP layer, calls `handle_api_send`, and maps a raised `AppError` to its status
code, so a single `add_main_api_routes` assembles the whole service. Finally
[bin/server.rs](bin/server.rs) constructs a `MockApp`, builds the router, and serves it —
completing the path from a request on the wire, through the decode-authenticate-handle-encode
pipeline the namespace wired, to a JSON response or a status-coded error.

## The payoff

Read the wiring in [src/contexts/app.rs](src/contexts/app.rs) as the whole specification of
this application, and the shape of CGP comes into focus. Every endpoint, wrapper, backend,
and type is a swappable part, and the context's wiring is the single place that chooses among
them. Adding an endpoint means adding a handler provider and one line to `DefaultApiHandlers`.
Moving to a database means writing one backend provider and changing one wiring entry. In
both cases the interfaces, the endpoints, and the wrappers are untouched — the modularity is
the reason CGP exists, and this small service is a template for any request/response backend
whose endpoints share cross-cutting concerns and whose implementation should stay swappable
behind abstract types.
