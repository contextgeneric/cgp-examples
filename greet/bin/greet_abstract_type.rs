use core::fmt::Display;

use cgp::prelude::*;

/// Greets a person.
#[cgp_component(Greeter)]
pub trait CanGreet {
    fn greet(&self);
}

/// A context's abstract name type.
#[cgp_type]
pub trait HasNameType {
    type Name;
}

/// Greets by printing `Hello, {name}!` for the context's abstract `Name` type.
#[cgp_impl(new GreetHello)]
#[use_type(HasNameType.Name)]
impl Greeter
where
    Name: Display,
{
    fn greet(&self, #[implicit] name: &Name) {
        println!("Hello, {name}!");
    }
}

#[derive(HasField)]
pub struct Person {
    pub name: String,
}

impl HasNameType for Person {
    type Name = String;
}

delegate_components! {
    Person {
        GreeterComponent: GreetHello,
    }
}

fn main() {
    let person = Person {
        name: "Alice".into(),
    };

    person.greet();
}
