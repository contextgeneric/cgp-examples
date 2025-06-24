use alloc::boxed::Box;

#[derive(Debug, Eq, PartialEq)]
pub struct Times<Expr>(pub Box<Expr>, pub Box<Expr>);
