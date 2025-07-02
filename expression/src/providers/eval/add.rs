use core::ops::Add;

use cgp::extra::handler::CanCompute;
use cgp::prelude::*;

use crate::types::Plus;

#[cgp_new_provider]
impl<Context, Code, MathExpr, Output> Computer<Context, Code, Plus<MathExpr>> for EvalAdd
where
    Context: CanCompute<Code, MathExpr, Output = Output>,
    Output: Add<Output = Output>,
{
    type Output = Output;

    fn compute(
        context: &Context,
        code: PhantomData<Code>,
        Plus { left, right }: Plus<MathExpr>,
    ) -> Self::Output {
        let output_a = context.compute(code, *left);
        let output_b = context.compute(code, *right);

        output_a + output_b
    }
}
