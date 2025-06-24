use alloc::boxed::Box;

use cgp::prelude::*;

#[derive(Debug, Eq, PartialEq, HasField)]
pub struct Plus<Expr>(pub Box<Expr>, pub Box<Expr>);
