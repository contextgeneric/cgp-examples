use core::ops::Neg;

use cgp::extra::handler::{CanComputeRef, ComputerRef, ComputerRefComponent};
use cgp::prelude::*;

use crate::types::Negate;

/// Evaluates a `Negate` by negating the value its operand evaluates to. See
/// [the add_mult_neg example](https://contextgeneric.dev/docs/projects/cgp-examples/expression/examples/add-mult-neg).
#[cgp_impl(new EvalNegate)]
#[uses(CanComputeRef<Code, MathExpr, Output = Output>)]
impl<Code, MathExpr, Output> ComputerRef<Code, Negate<MathExpr>>
where
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
