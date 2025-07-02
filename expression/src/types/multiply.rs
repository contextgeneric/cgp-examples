use alloc::boxed::Box;

use cgp::prelude::*;

#[derive(Debug, Eq, PartialEq, HasField)]
pub struct Times<Expr> {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}
