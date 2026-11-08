use cgp::extra::handler::{ComputerRef, ComputerRefComponent};
use cgp::prelude::*;

use crate::types::Literal;

/// Evaluates a `Literal` to its value. See
/// [the add_mult example](https://contextgeneric.dev/docs/projects/cgp-examples/expression/examples/add-mult).
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
