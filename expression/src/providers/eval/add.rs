use core::ops::Add;

use cgp::extra::handler::{
    CanCompute, CanComputeRef, ComputerComponent, ComputerRef, ComputerRefComponent,
};
use cgp::prelude::*;

use crate::types::Plus;

#[cgp_impl(new EvalAdd)]
#[uses(CanCompute<Code, MathExpr, Output = Output>)]
impl<Code, MathExpr, Output> Computer<Code, Plus<MathExpr>>
where
    Output: Add<Output = Output>,
{
    type Output = Output;

    fn compute(
        &self,
        code: PhantomData<Code>,
        Plus { left, right }: Plus<MathExpr>,
    ) -> Self::Output {
        let output_a = self.compute(code, *left);
        let output_b = self.compute(code, *right);

        output_a + output_b
    }
}

#[cgp_impl(EvalAdd)]
#[uses(CanComputeRef<Code, MathExpr, Output = Output>)]
impl<Code, MathExpr, Output> ComputerRef<Code, Plus<MathExpr>>
where
    Output: Add<Output = Output>,
{
    type Output = Output;

    fn compute_ref(
        &self,
        code: PhantomData<Code>,
        Plus { left, right }: &Plus<MathExpr>,
    ) -> Self::Output {
        let output_a = self.compute_ref(code, left);
        let output_b = self.compute_ref(code, right);

        output_a + output_b
    }
}
