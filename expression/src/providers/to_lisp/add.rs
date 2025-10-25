use alloc::borrow::ToOwned;
use alloc::vec;

use cgp::core::field::impls::CanUpcast;
use cgp::extra::handler::{CanComputeRef, ComputerRef, ComputerRefComponent};
use cgp::prelude::*;

use crate::components::HasLispExprType;
use crate::types::{Ident, List, Plus};

#[derive(HasFields, ExtractField, FromVariant)]
enum LispSubExpr<Expr> {
    List(List<Expr>),
    Ident(Ident),
}

#[cgp_impl(new PlusToLisp)]
impl<Context, Code, MathExpr, LispExpr> ComputerRef<Code, Plus<MathExpr>> for Context
where
    Context:
        HasLispExprType<LispExpr = LispExpr> + CanComputeRef<Code, MathExpr, Output = LispExpr>,
    LispSubExpr<LispExpr>: CanUpcast<LispExpr>,
{
    type Output = LispExpr;

    fn compute_ref(
        context: &Context,
        code: PhantomData<Code>,
        Plus { left, right }: &Plus<MathExpr>,
    ) -> Self::Output {
        let expr_a = context.compute_ref(code, left);
        let expr_b = context.compute_ref(code, right);
        let ident = LispSubExpr::Ident(Ident("+".to_owned())).upcast(PhantomData);

        LispSubExpr::List(List(vec![ident.into(), expr_a.into(), expr_b.into()]))
            .upcast(PhantomData)
    }
}
