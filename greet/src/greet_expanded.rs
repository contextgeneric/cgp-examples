//! A hand-written expansion of what `#[cgp_component]` generates for `CanGreet`.
//!
//! The items here are the macro's output written out by hand, so a reader can see the wiring
//! machinery the component macro adds: the component marker, the provider trait, and the two
//! blanket impls that connect them. They carry no docs of their own, and the file differs from
//! the macro's current output in the details the knowledge base records.

use cgp::prelude::*;

pub struct GreeterComponent;

pub trait CanGreet {
    fn greet(&self);
}

pub trait Greeter<Context>: IsProviderFor<GreeterComponent, Context, ()> {
    fn greet(context: &Context);
}

impl<Context> CanGreet for Context
where
    Context: DelegateComponent<GreeterComponent>,
    Context::Delegate: Greeter<Context>,
{
    fn greet(&self) {
        Context::Delegate::greet(self)
    }
}

impl<Component, Context> Greeter<Context> for Component
where
    Component: DelegateComponent<GreeterComponent> + IsProviderFor<GreeterComponent, Context, ()>,
    Component::Delegate: Greeter<Context>,
{
    fn greet(context: &Context) {
        Component::Delegate::greet(context)
    }
}
