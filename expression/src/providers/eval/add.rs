use core::ops::Add;

use cgp::extra::handler::CanCompute;
use cgp::prelude::*;

use crate::types::Plus;

#[cgp_new_provider]
impl<Context, Code, Expr, Output> Computer<Context, Code, Plus<Expr>> for EvalAdd
where
    Context: CanCompute<Code, Expr, Output = Output>,
    Output: Add<Output = Output>,
{
    type Output = Output;

    fn compute(
        context: &Context,
        code: PhantomData<Code>,
        Plus(expr_a, expr_b): Plus<Expr>,
    ) -> Self::Output {
        let output_a = context.compute(code, *expr_a);
        let output_b = context.compute(code, *expr_b);

        output_a + output_b
    }
}
