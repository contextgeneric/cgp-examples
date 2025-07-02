use alloc::boxed::Box;

use cgp::prelude::*;

#[derive(Debug, Eq, PartialEq, HasField)]
pub struct Plus<Expr> {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}
