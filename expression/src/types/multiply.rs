use alloc::boxed::Box;

use cgp::prelude::*;

#[derive(Debug, Eq, PartialEq, HasField)]
pub struct Times<Expr>(pub Box<Expr>, pub Box<Expr>);
