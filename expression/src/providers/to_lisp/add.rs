use alloc::borrow::ToOwned;
use alloc::vec;
use cgp::extra::handler::CanCompute;
use cgp::prelude::*;

use crate::components::HasLispExprType;
use crate::types::{Ident, List, Plus};

#[cgp_new_provider]
impl<Context, Code, Expr, LispExpr> Computer<Context, Code, Plus<Expr>> for AddToLisp
where
    Context: HasLispExprType<LispExpr = LispExpr> + CanCompute<Code, Expr, Output = LispExpr>,
    LispExpr: FromVariant<symbol!("List"), Value = List<LispExpr>>
        + FromVariant<symbol!("Ident"), Value = Ident>,
{
    type Output = LispExpr;

    fn compute(
        context: &Context,
        code: PhantomData<Code>,
        Plus(expr_a, expr_b): Plus<Expr>,
    ) -> Self::Output {
        let expr_a = context.compute(code, *expr_a);
        let expr_b = context.compute(code, *expr_b);
        let ident = LispExpr::from_variant(PhantomData::<symbol!("Ident")>, Ident("+".to_owned()));

        LispExpr::from_variant(
            PhantomData::<symbol!("List")>,
            List(vec![ident.into(), expr_a.into(), expr_b.into()]),
        )
    }
}
