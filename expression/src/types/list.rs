use alloc::boxed::Box;
use alloc::vec::Vec;

#[derive(Debug, Eq, PartialEq)]
pub struct List<Expr>(pub Vec<Box<Expr>>);
