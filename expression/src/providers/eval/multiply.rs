use core::ops::Mul;

use cgp::extra::handler::{CanCompute, CanComputeRef, ComputerRef, ComputerRefComponent};
use cgp::prelude::*;

use crate::types::Times;

#[cgp_impl(new EvalMultiply)]
#[uses(CanCompute<Code, MathExpr, Output = Output>)]
impl<Code, MathExpr, Output> Computer<Code, Times<MathExpr>>
where
    Output: Mul<Output = Output>,
{
    type Output = Output;

    fn compute(&self, code: PhantomData<Code>, Times { left, right }: Times<MathExpr>) -> Output {
        let output_a = self.compute(code, *left);
        let output_b = self.compute(code, *right);

        output_a * output_b
    }
}

#[cgp_impl(EvalMultiply)]
#[uses(CanComputeRef<Code, MathExpr, Output = Output>)]
impl<Code, MathExpr, Output> ComputerRef<Code, Times<MathExpr>>
where
    Output: Mul<Output = Output>,
{
    type Output = Output;

    fn compute_ref(
        &self,
        code: PhantomData<Code>,
        Times { left, right }: &Times<MathExpr>,
    ) -> Self::Output {
        let output_a = self.compute_ref(code, left);
        let output_b = self.compute_ref(code, right);

        output_a * output_b
    }
}
