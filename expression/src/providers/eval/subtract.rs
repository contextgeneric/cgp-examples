use core::ops::Sub;

use cgp::extra::handler::{CanCompute, CanComputeRef, ComputerRef, ComputerRefComponent};
use cgp::prelude::*;

use crate::types::{Minus, Negate, Plus};

#[cgp_impl(new EvalSubtract)]
impl<Context, Code, MathExpr, Output> ComputerRef<Code, Minus<MathExpr>> for Context
where
    Context: CanComputeRef<Code, MathExpr, Output = Output>,
    Output: Sub<Output = Output>,
{
    type Output = Output;

    fn compute_ref(
        context: &Context,
        code: PhantomData<Code>,
        Minus { left, right }: &Minus<MathExpr>,
    ) -> Self::Output {
        let output_a = context.compute_ref(code, left);
        let output_b = context.compute_ref(code, right);

        output_a - output_b
    }
}

#[cgp_impl(new EvalSubtractWithNegate)]
impl<Context, Code, Expr, Output> Computer<Code, Minus<Expr>> for Context
where
    Context: CanCompute<Code, Plus<Expr>, Output = Output>,
    Expr: FromVariant<Symbol!("Negate"), Value = Negate<Expr>>,
{
    type Output = Output;

    fn compute(
        context: &Context,
        code: PhantomData<Code>,
        Minus { left, right }: Minus<Expr>,
    ) -> Self::Output {
        let expr_c = Expr::from_variant(PhantomData::<Symbol!("Negate")>, Negate(right));
        let add_expr = Plus {
            left,
            right: expr_c.into(),
        };

        context.compute(code, add_expr)
    }
}
