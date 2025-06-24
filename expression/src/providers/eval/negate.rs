use cgp::extra::handler::CanCompute;
use cgp::prelude::*;

use crate::types::Negate;

#[cgp_new_provider]
impl<Context, Code, Expr, Output> Computer<Context, Code, Negate<Expr>> for EvalNegate
where
    Context: CanCompute<Code, Expr, Output = Output>,
    Output: core::ops::Neg<Output = Output>,
{
    type Output = Output;

    fn compute(
        context: &Context,
        code: PhantomData<Code>,
        Negate(expr): Negate<Expr>,
    ) -> Self::Output {
        let output = context.compute(code, *expr);

        -output
    }
}
