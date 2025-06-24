use alloc::boxed::Box;

#[derive(Debug, Eq, PartialEq)]
pub struct Plus<Expr>(pub Box<Expr>, pub Box<Expr>);
