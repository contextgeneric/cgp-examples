use core::ops::Add;

use cgp::extra::handler::{
    CanCompute, CanComputeRef, ComputerComponent, ComputerRef, ComputerRefComponent,
};
use cgp::prelude::*;

use crate::types::Plus;

#[cgp_impl(new EvalAdd)]
impl<Context, Code, MathExpr, Output> Computer<Code, Plus<MathExpr>> for Context
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

#[cgp_impl(EvalAdd)]
impl<Context, Code, MathExpr, Output> ComputerRef<Code, Plus<MathExpr>> for Context
where
    Context: CanComputeRef<Code, MathExpr, Output = Output>,
    Output: Add<Output = Output>,
{
    type Output = Output;

    fn compute_ref(
        context: &Context,
        code: PhantomData<Code>,
        Plus { left, right }: &Plus<MathExpr>,
    ) -> Self::Output {
        let output_a = context.compute_ref(code, left);
        let output_b = context.compute_ref(code, right);

        output_a + output_b
    }
}
