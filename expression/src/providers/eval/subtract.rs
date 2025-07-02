use cgp::extra::handler::CanCompute;
use cgp::prelude::*;

use crate::types::{Minus, Negate, Plus};

#[cgp_new_provider]
impl<Context, Code, MathExpr, Output> Computer<Context, Code, Minus<MathExpr>> for EvalSubtract
where
    Context: CanCompute<Code, MathExpr, Output = Output>,
    Output: core::ops::Sub<Output = Output>,
{
    type Output = Output;

    fn compute(
        context: &Context,
        code: PhantomData<Code>,
        Minus { left, right }: Minus<MathExpr>,
    ) -> Self::Output {
        let output_a = context.compute(code, *left);
        let output_b = context.compute(code, *right);

        output_a - output_b
    }
}

#[cgp_new_provider]
impl<Context, Code, Expr, Output> Computer<Context, Code, Minus<Expr>> for EvalSubtractWithNegate
where
    Context: CanCompute<Code, Plus<Expr>, Output = Output>,
    Expr: FromVariant<symbol!("Negate"), Value = Negate<Expr>>,
{
    type Output = Output;

    fn compute(
        context: &Context,
        code: PhantomData<Code>,
        Minus { left, right }: Minus<Expr>,
    ) -> Self::Output {
        let expr_c = Expr::from_variant(PhantomData::<symbol!("Negate")>, Negate(right));
        let add_expr = Plus {
            left,
            right: expr_c.into(),
        };

        context.compute(code, add_expr)
    }
}
