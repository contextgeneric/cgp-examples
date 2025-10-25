use core::ops::Neg;

use cgp::extra::handler::{CanComputeRef, ComputerRef, ComputerRefComponent};
use cgp::prelude::*;

use crate::types::Negate;

#[cgp_impl(new EvalNegate)]
impl<Context, Code, MathExpr, Output> ComputerRef<Code, Negate<MathExpr>> for Context
where
    Context: CanComputeRef<Code, MathExpr, Output = Output>,
    Output: Neg<Output = Output>,
{
    type Output = Output;

    fn compute_ref(
        context: &Context,
        code: PhantomData<Code>,
        Negate(expr): &Negate<MathExpr>,
    ) -> Self::Output {
        -context.compute_ref(code, expr)
    }
}
