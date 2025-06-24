use cgp::extra::handler::CanCompute;
use cgp::prelude::*;

use crate::types::Add;

#[cgp_new_provider]
impl<Context, Code, Expr, Output> Computer<Context, Code, Add<Expr>> for EvalAdd
where
    Context: CanCompute<Code, Expr, Output = Output>,
    Output: core::ops::Add<Output = Output>,
{
    type Output = Output;

    fn compute(
        context: &Context,
        code: PhantomData<Code>,
        Add(expr_a, expr_b): Add<Expr>,
    ) -> Self::Output {
        let output_a = context.compute(code, *expr_a);
        let output_b = context.compute(code, *expr_b);

        output_a + output_b
    }
}
