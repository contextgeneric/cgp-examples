use core::fmt::Display;

use cgp::prelude::*;

#[cgp_component(Greeter)]
pub trait CanGreet {
    fn greet(&self);
}

#[cgp_type]
pub trait HasNameType {
    type Name;
}

#[cgp_auto_getter]
pub trait HasName: HasNameType {
    fn name(&self) -> &Self::Name;
}

#[cgp_new_provider]
impl<Context> Greeter<Context> for GreetHello
where
    Context: HasName,
    Context::Name: Display,
{
    fn greet(context: &Context) {
        println!("Hello, {}!", context.name());
    }
}

#[cgp_context]
#[derive(HasField)]
pub struct Person {
    pub name: String,
}

delegate_components! {
    PersonComponents {
        NameTypeProviderComponent: UseType<String>,
        GreeterComponent: GreetHello,
    }
}

fn main() {
    let person = Person {
        name: "Alice".into(),
    };

    person.greet();
}
