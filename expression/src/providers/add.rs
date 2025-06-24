use cgp::prelude::*;

#[cgp_new_provider]
impl<Context, Code, Expr, Output> Computer<Context, Code, Add<Expr>> for HandleAdd where
    Context: HasExpressionType<Expression = Expr> + CanCompute<Code, Expr, Output = Output>
{
}
