use alloc::boxed::Box;

#[derive(Debug, Eq, PartialEq)]
pub struct Negate<Expr>(pub Box<Expr>);
