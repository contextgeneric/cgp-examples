use alloc::boxed::Box;

#[derive(Debug, Eq, PartialEq)]
pub struct Minus<Expr>(pub Box<Expr>, pub Box<Expr>);
