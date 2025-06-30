use alloc::borrow::ToOwned;
use alloc::vec;
use cgp::extra::handler::{CanComputeRef, ComputerRef, ComputerRefComponent};
use cgp::prelude::*;

use crate::components::HasLispExprType;
use crate::types::{Ident, List, Times};

#[cgp_new_provider]
impl<Context, Code, Expr, LispExpr> ComputerRef<Context, Code, Times<Expr>> for MultiplyToLisp
where
    Context: HasLispExprType<LispExpr = LispExpr> + CanComputeRef<Code, Expr, Output = LispExpr>,
    LispExpr: FromVariant<symbol!("List"), Value = List<LispExpr>>
        + FromVariant<symbol!("Ident"), Value = Ident>,
{
    type Output = LispExpr;

    fn compute_ref(
        context: &Context,
        code: PhantomData<Code>,
        Times(expr_a, expr_b): &Times<Expr>,
    ) -> Self::Output {
        let expr_a = context.compute_ref(code, expr_a);
        let expr_b = context.compute_ref(code, expr_b);
        let ident = LispExpr::from_variant(PhantomData::<symbol!("Ident")>, Ident("*".to_owned()));

        LispExpr::from_variant(
            PhantomData::<symbol!("List")>,
            List(vec![ident.into(), expr_a.into(), expr_b.into()]),
        )
    }
}
