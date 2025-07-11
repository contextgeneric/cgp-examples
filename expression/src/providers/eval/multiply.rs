use core::ops::Mul;

use cgp::extra::handler::{CanCompute, CanComputeRef, ComputerRef, ComputerRefComponent};
use cgp::prelude::*;

use crate::types::Times;

#[cgp_new_provider]
impl<Context, Code, MathExpr, Output> Computer<Context, Code, Times<MathExpr>> for EvalMultiply
where
    Context: CanCompute<Code, MathExpr, Output = Output>,
    Output: Mul<Output = Output>,
{
    type Output = Output;

    fn compute(
        context: &Context,
        code: PhantomData<Code>,
        Times { left, right }: Times<MathExpr>,
    ) -> Output {
        let output_a = context.compute(code, *left);
        let output_b = context.compute(code, *right);

        output_a * output_b
    }
}

#[cgp_provider]
impl<Context, Code, MathExpr, Output> ComputerRef<Context, Code, Times<MathExpr>> for EvalMultiply
where
    Context: CanComputeRef<Code, MathExpr, Output = Output>,
    Output: Mul<Output = Output>,
{
    type Output = Output;

    fn compute_ref(
        context: &Context,
        code: PhantomData<Code>,
        Times { left, right }: &Times<MathExpr>,
    ) -> Self::Output {
        let output_a = context.compute_ref(code, left);
        let output_b = context.compute_ref(code, right);

        output_a * output_b
    }
}
