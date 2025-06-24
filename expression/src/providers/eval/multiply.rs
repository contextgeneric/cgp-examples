use cgp::extra::handler::CanCompute;
use cgp::prelude::*;

use crate::types::Times;

#[cgp_new_provider]
impl<Context, Code, Expr, Output> Computer<Context, Code, Times<Expr>> for EvalMultiply
where
    Context: CanCompute<Code, Expr, Output = Output>,
    Output: core::ops::Mul<Output = Output>,
{
    type Output = Output;

    fn compute(
        context: &Context,
        code: PhantomData<Code>,
        Times(expr_a, expr_b): Times<Expr>,
    ) -> Output {
        let output_a = context.compute(code, *expr_a);
        let output_b = context.compute(code, *expr_b);

        output_a * output_b
    }
}
