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
    Context: HasCgpProvider,
    Context::CgpProvider: Greeter<Context>,
{
    fn greet(&self) {
        Context::CgpProvider::greet(self)
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
