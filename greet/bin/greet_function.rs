use cgp::prelude::*;

/// Greets a person by name.
#[cgp_fn]
pub fn greet(&self, #[implicit] name: &str) {
    println!("Hello, {name}!");
}

#[derive(HasField)]
pub struct Person {
    pub name: String,
}

fn main() {
    let person = Person {
        name: "Alice".to_owned(),
    };

    person.greet();
}
