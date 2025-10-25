use cgp::core::field::impls::CanUpcast;
use cgp::extra::handler::{ComputerRef, ComputerRefComponent};
use cgp::prelude::*;

use crate::components::HasLispExprType;
use crate::types::Literal;

#[derive(HasFields, ExtractField, FromVariant)]
enum LispSubExpr<T> {
    Literal(Literal<T>),
}

#[cgp_impl(new LiteralToLisp)]
impl<Context, Code, T, LispExpr> ComputerRef<Code, Literal<T>> for Context
where
    Context: HasLispExprType<LispExpr = LispExpr>,
    LispSubExpr<T>: CanUpcast<LispExpr>,
    T: Clone,
{
    type Output = LispExpr;

    fn compute_ref(
        _context: &Context,
        _code: PhantomData<Code>,
        Literal(value): &Literal<T>,
    ) -> Self::Output {
        LispSubExpr::Literal(Literal(value.clone())).upcast(PhantomData)
    }
}
