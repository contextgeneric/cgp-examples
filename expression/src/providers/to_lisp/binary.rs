use core::fmt::Display;

use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::vec;
use cgp::extra::handler::CanCompute;
use cgp::prelude::*;

use crate::components::HasLispExprType;
use crate::types::{Ident, List};

#[cgp_new_provider]
impl<'a, Context, Code, Expr, SubExpr, LispExpr, Operator> Computer<Context, Code, &'a SubExpr>
    for BinaryOpToLisp<Operator>
where
    Context: HasLispExprType<LispExpr = LispExpr>
        + for<'b> CanCompute<Code, &'b Expr, Output = LispExpr>,
    SubExpr: HasField<Index<0>, Value = Box<Expr>> + HasField<Index<1>, Value = Box<Expr>>,
    LispExpr: FromVariant<symbol!("List"), Value = List<LispExpr>>
        + FromVariant<symbol!("Ident"), Value = Ident>,
    Operator: Default + Display,
{
    type Output = LispExpr;

    fn compute(context: &Context, code: PhantomData<Code>, expr: &SubExpr) -> Self::Output {
        let expr_a = context.compute(code, expr.get_field(PhantomData::<Index<0>>).as_ref());
        let expr_b = context.compute(code, expr.get_field(PhantomData::<Index<1>>).as_ref());
        let ident = LispExpr::from_variant(
            PhantomData::<symbol!("Ident")>,
            Ident(Operator::default().to_string()),
        );

        LispExpr::from_variant(
            PhantomData::<symbol!("List")>,
            List(vec![ident.into(), expr_a.into(), expr_b.into()]),
        )
    }
}
