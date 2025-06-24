use cgp::extra::handler::CanCompute;
use cgp::prelude::*;

use crate::components::HasExpressionType;
use crate::types::Multiply;

#[cgp_new_provider]
impl<Context, Code, Expr, Output> Computer<Context, Code, Multiply<Expr>> for EvalMultiply
where
    Context: HasExpressionType<Expression = Expr> + CanCompute<Code, Expr, Output = Output>,
    Output: core::ops::Mul<Output = Output>,
{
    type Output = Output;

    fn compute(
        context: &Context,
        code: PhantomData<Code>,
        Multiply(expr_a, expr_b): Multiply<Expr>,
    ) -> Output {
        let output_a = context.compute(code, *expr_a);
        let output_b = context.compute(code, *expr_b);

        output_a * output_b
    }
}
