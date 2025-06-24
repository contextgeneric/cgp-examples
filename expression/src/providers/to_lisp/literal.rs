use cgp::prelude::*;

use crate::components::HasLispExprType;
use crate::types::Literal;

#[cgp_new_provider]
impl<Context, Code, T, LispExpr> Computer<Context, Code, Literal<T>> for LiteralToLisp
where
    Context: HasLispExprType<LispExpr = LispExpr>,
    LispExpr: FromVariant<symbol!("Literal"), Value = Literal<T>>,
{
    type Output = LispExpr;

    fn compute(_context: &Context, _code: PhantomData<Code>, value: Literal<T>) -> Self::Output {
        LispExpr::from_variant(PhantomData, value)
    }
}
