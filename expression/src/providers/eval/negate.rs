use core::ops::Neg;

use cgp::extra::handler::{CanComputeRef, ComputerRef, ComputerRefComponent};
use cgp::prelude::*;

use crate::types::Negate;

#[cgp_impl(new EvalNegate)]
impl<Code, MathExpr, Output> ComputerRef<Code, Negate<MathExpr>>
where
    Self: CanComputeRef<Code, MathExpr, Output = Output>,
    Output: Neg<Output = Output>,
{
    type Output = Output;

    fn compute_ref(
        &self,
        code: PhantomData<Code>,
        Negate(expr): &Negate<MathExpr>,
    ) -> Self::Output {
        -self.compute_ref(code, expr)
    }
}
