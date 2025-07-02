use alloc::boxed::Box;

#[derive(Debug, Eq, PartialEq)]
pub struct Minus<Expr> {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}
