use cgp::extra::handler::{ComputerRef, ComputerRefComponent};
use cgp::prelude::*;

use crate::components::HasLispExprType;
use crate::types::Literal;

#[cgp_new_provider]
impl<Context, Code, T, LispExpr> ComputerRef<Context, Code, Literal<T>> for LiteralToLisp
where
    Context: HasLispExprType<LispExpr = LispExpr>,
    LispExpr: FromVariant<symbol!("Literal"), Value = Literal<T>>,
    T: Clone,
{
    type Output = LispExpr;

    fn compute_ref(
        _context: &Context,
        _code: PhantomData<Code>,
        Literal(value): &Literal<T>,
    ) -> Self::Output {
        LispExpr::from_variant(PhantomData, Literal(value.clone()))
    }
}
