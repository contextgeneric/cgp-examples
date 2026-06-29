pub struct Email(pub String);

pub struct User;
pub struct UserData;
pub struct UserId;

pub struct Post;
pub struct PostId;

pub enum Error {
    InvalidUsername,
    InvalidMessage,
}

pub struct PostgresDb;

#[derive(PartialOrd, PartialEq)]
pub struct Probability(pub f64);

impl Probability {
    pub const fn new(value: f64) -> Self {
        Self(value)
    }
}
