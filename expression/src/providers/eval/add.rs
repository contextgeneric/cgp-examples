use cgp::extra::handler::CanCompute;
use cgp::prelude::*;

use crate::components::HasExpressionType;
use crate::contexts::add_mult::Expr;
use crate::types::Add;

#[cgp_new_provider]
impl<Context, Code> Computer<Context, Code, Add<Expr>> for EvalAdd
where
    Context: HasExpressionType<Expression = Expr> + CanCompute<Code, Expr>,
    // Output: core::ops::Add<Output = Output>,
{
    type Output = u64;

    fn compute(
        context: &Context,
        code: PhantomData<Code>,
        Add(expr_a, expr_b): Add<Expr>,
    ) -> Self::Output {
        todo!()
        // let output_a = context.compute(code, *expr_a);
        // let output_b = context.compute(code, *expr_b);

        // output_a + output_b
    }
}
