use cgp::extra::handler::{ComputerRef, ComputerRefComponent};
use cgp::prelude::*;

use crate::types::Literal;

#[cgp_new_provider]
impl<Context, Code, T> Computer<Context, Code, Literal<T>> for EvalLiteral {
    type Output = T;

    fn compute(_context: &Context, _code: PhantomData<Code>, Literal(value): Literal<T>) -> T {
        value
    }
}

#[cgp_provider]
impl<Context, Code, T> ComputerRef<Context, Code, Literal<T>> for EvalLiteral
where
    T: Clone,
{
    type Output = T;

    fn compute_ref(_context: &Context, _code: PhantomData<Code>, Literal(value): &Literal<T>) -> T {
        value.clone()
    }
}
