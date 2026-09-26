use cgp::core::field::impls::CanUpcast;
use cgp::extra::handler::{ComputerRef, ComputerRefComponent};
use cgp::prelude::*;

use crate::components::HasLispExprType;
use crate::types::Literal;

#[derive(CgpData)]
enum LispSubExpr<T> {
    Literal(Literal<T>),
}

#[cgp_impl(new LiteralToLisp)]
#[use_type(HasLispExprType.LispExpr)]
impl<Code, T> ComputerRef<Code, Literal<T>>
where
    LispSubExpr<T>: CanUpcast<LispExpr>,
    T: Clone,
{
    type Output = LispExpr;

    fn compute_ref(&self, _code: PhantomData<Code>, Literal(value): &Literal<T>) -> Self::Output {
        LispSubExpr::Literal(Literal(value.clone())).upcast(PhantomData)
    }
}
