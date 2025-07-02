use alloc::borrow::ToOwned;
use alloc::vec;
use cgp::core::field::CanUpcast;
use cgp::extra::handler::{CanComputeRef, ComputerRef, ComputerRefComponent};
use cgp::prelude::*;

use crate::components::HasLispExprType;
use crate::types::{Ident, List, Times};

#[derive(HasFields, ExtractField, FromVariant)]
enum LispSubExpr<Expr> {
    List(List<Expr>),
    Ident(Ident),
}

#[cgp_new_provider]
impl<Context, Code, Expr, LispExpr> ComputerRef<Context, Code, Times<Expr>> for TimesToLisp
where
    Context: HasLispExprType<LispExpr = LispExpr> + CanComputeRef<Code, Expr, Output = LispExpr>,
    LispSubExpr<LispExpr>: CanUpcast<LispExpr>,
{
    type Output = LispExpr;

    fn compute_ref(
        context: &Context,
        code: PhantomData<Code>,
        Times(expr_a, expr_b): &Times<Expr>,
    ) -> Self::Output {
        let expr_a = context.compute_ref(code, expr_a);
        let expr_b = context.compute_ref(code, expr_b);
        let ident = LispSubExpr::Ident(Ident("*".to_owned())).upcast(PhantomData);

        LispSubExpr::List(List(vec![ident.into(), expr_a.into(), expr_b.into()]))
            .upcast(PhantomData)
    }
}
