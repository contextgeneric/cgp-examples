use cgp::extra::handler::{CanComputeRef, ComputerRef, ComputerRefComponent};
use cgp::prelude::*;

use crate::types::Negate;

#[cgp_new_provider]
impl<Context, Code, MathExpr, Output> ComputerRef<Context, Code, Negate<MathExpr>> for EvalNegate
where
    Context: CanComputeRef<Code, MathExpr, Output = Output>,
    Output: core::ops::Neg<Output = Output>,
{
    type Output = Output;

    fn compute_ref(
        context: &Context,
        code: PhantomData<Code>,
        Negate(expr): &Negate<MathExpr>,
    ) -> Self::Output {
        let output = context.compute_ref(code, expr);

        -output
    }
}
