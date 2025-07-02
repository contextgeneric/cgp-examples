use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};

pub enum Expr {
    Plus(Box<Expr>, Box<Expr>),
    Times(Box<Expr>, Box<Expr>),
    Literal(u64),
}

pub fn eval(expr: Expr) -> u64 {
    match expr {
        Expr::Plus(a, b) => eval(*a) + eval(*b),
        Expr::Times(a, b) => eval(*a) * eval(*b),
        Expr::Literal(value) => value,
    }
}

pub fn expr_to_string(expr: &Expr) -> String {
    match expr {
        Expr::Plus(a, b) => format!("({} + {})", expr_to_string(a), expr_to_string(b)),
        Expr::Times(a, b) => format!("({} * {})", expr_to_string(a), expr_to_string(b)),
        Expr::Literal(value) => value.to_string(),
    }
}
