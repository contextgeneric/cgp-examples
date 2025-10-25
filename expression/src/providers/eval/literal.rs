use cgp::extra::handler::{ComputerRef, ComputerRefComponent};
use cgp::prelude::*;

use crate::types::Literal;

#[cgp_impl(new EvalLiteral )]
impl<Context, Code, T> Computer<Code, Literal<T>> for Context {
    type Output = T;

    fn compute(_context: &Context, _code: PhantomData<Code>, Literal(value): Literal<T>) -> T {
        value
    }
}

#[cgp_impl(EvalLiteral)]
impl<Context, Code, T> ComputerRef<Code, Literal<T>> for Context
where
    T: Clone,
{
    type Output = T;

    fn compute_ref(_context: &Context, _code: PhantomData<Code>, Literal(value): &Literal<T>) -> T {
        value.clone()
    }
}
