use cgp::extra::handler::{ComputerRef, ComputerRefComponent};
use cgp::prelude::*;

use crate::types::Literal;

#[cgp_impl(new EvalLiteral)]
impl<Code, T> Computer<Code, Literal<T>> {
    type Output = T;

    fn compute(&self, _code: PhantomData<Code>, Literal(value): Literal<T>) -> T {
        value
    }
}

#[cgp_impl(EvalLiteral)]
impl<Code, T> ComputerRef<Code, Literal<T>>
where
    T: Clone,
{
    type Output = T;

    fn compute_ref(&self, _code: PhantomData<Code>, Literal(value): &Literal<T>) -> T {
        value.clone()
    }
}
