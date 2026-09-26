use alloc::borrow::ToOwned;
use alloc::vec;

use cgp::core::field::impls::CanUpcast;
use cgp::extra::handler::{CanComputeRef, ComputerRef, ComputerRefComponent};
use cgp::prelude::*;

use crate::components::HasLispExprType;
use crate::types::{Ident, List, Times};

#[derive(CgpData)]
enum LispSubExpr<Expr> {
    List(List<Expr>),
    Ident(Ident),
}

#[cgp_impl(new TimesToLisp)]
#[use_type(HasLispExprType.LispExpr)]
#[uses(CanComputeRef<Code, MathExpr, Output = LispExpr>)]
impl<Code, MathExpr> ComputerRef<Code, Times<MathExpr>>
where
    LispSubExpr<LispExpr>: CanUpcast<LispExpr>,
{
    type Output = LispExpr;

    fn compute_ref(
        &self,
        code: PhantomData<Code>,
        Times { left, right }: &Times<MathExpr>,
    ) -> Self::Output {
        let expr_a = self.compute_ref(code, left);
        let expr_b = self.compute_ref(code, right);
        let ident = LispSubExpr::Ident(Ident("*".to_owned())).upcast(PhantomData);

        LispSubExpr::List(List(vec![ident.into(), expr_a.into(), expr_b.into()]))
            .upcast(PhantomData)
    }
}
