use cgp::prelude::*;

use crate::types::Literal;

#[cgp_new_provider]
impl<Context, Code, T> Computer<Context, Code, Literal<T>> for EvalLiteral {
    type Output = T;

    fn compute(_context: &Context, _code: PhantomData<Code>, Literal(value): Literal<T>) -> T {
        value
    }
}
