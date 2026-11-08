use core::ops::Sub;

use cgp::extra::handler::{CanCompute, CanComputeRef, ComputerRef, ComputerRefComponent};
use cgp::prelude::*;

use crate::types::{Minus, Negate, Plus};

/// Evaluates a `Minus` by subtracting the right operand's value from the left's. See
/// [the add_mult_neg example](https://contextgeneric.dev/docs/projects/cgp-examples/expression/examples/add-mult-neg).
#[cgp_impl(new EvalSubtract)]
#[uses(CanComputeRef<Code, MathExpr, Output = Output>)]
impl<Code, MathExpr, Output> ComputerRef<Code, Minus<MathExpr>>
where
    Output: Sub<Output = Output>,
{
    type Output = Output;

    fn compute_ref(
        &self,
        code: PhantomData<Code>,
        Minus { left, right }: &Minus<MathExpr>,
    ) -> Self::Output {
        let output_a = self.compute_ref(code, left);
        let output_b = self.compute_ref(code, right);

        output_a - output_b
    }
}

/// Evaluates a `Minus` by rewriting it as a `Plus` of the left operand and a negated right one.
#[cgp_impl(new EvalSubtractWithNegate)]
#[uses(CanCompute<Code, Plus<Expr>, Output = Output>)]
impl<Code, Expr, Output> Computer<Code, Minus<Expr>>
where
    Expr: FromVariant<Symbol!("Negate"), Value = Negate<Expr>>,
{
    type Output = Output;

    fn compute(&self, code: PhantomData<Code>, Minus { left, right }: Minus<Expr>) -> Self::Output {
        let expr_c = Expr::from_variant(PhantomData::<Symbol!("Negate")>, Negate(right));
        let add_expr = Plus {
            left,
            right: expr_c.into(),
        };

        self.compute(code, add_expr)
    }
}
