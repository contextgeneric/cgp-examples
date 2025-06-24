use cgp::prelude::*;

use crate::components::HasLispExprType;
use crate::types::Literal;

#[cgp_new_provider]
impl<'a, Context, Code, T, LispExpr> Computer<Context, Code, &'a Literal<T>> for LiteralToLisp
where
    Context: HasLispExprType<LispExpr = LispExpr>,
    LispExpr: FromVariant<symbol!("Literal"), Value = Literal<T>>,
    T: Clone,
{
    type Output = LispExpr;

    fn compute(
        _context: &Context,
        _code: PhantomData<Code>,
        Literal(value): &Literal<T>,
    ) -> Self::Output {
        LispExpr::from_variant(PhantomData, Literal(value.clone()))
    }
}
