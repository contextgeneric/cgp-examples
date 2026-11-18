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

/// Greets by printing `Hello, {name}!` from the context's `name` field.
#[cgp_impl(new GreetHello)]
impl Greeter {
    fn greet(&self, #[implicit] name: &str) {
        println!("Hello, {name}!");
    }
}

/// Greets by printing `Hi, {name}!` from the context's `name` field.
#[cgp_impl(new GreetHi)]
impl Greeter {
    fn greet(&self, #[implicit] name: &str) {
        println!("Hi, {name}!");
    }
}

#[derive(HasField)]
pub struct Person {
    pub name: String,
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
