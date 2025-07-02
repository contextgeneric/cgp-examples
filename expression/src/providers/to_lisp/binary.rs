use core::fmt::Display;

use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::vec;
use cgp::core::field::CanUpcast;
use cgp::extra::handler::{CanComputeRef, ComputerRef, ComputerRefComponent};
use cgp::prelude::*;

use crate::components::{HasExprType, HasLispExprType};
use crate::types::{Ident, List};

#[cgp_auto_getter]
pub trait BinarySubExpression<Expr> {
    fn left(&self) -> &Box<Expr>;
    fn right(&self) -> &Box<Expr>;
}

#[derive(HasFields, ExtractField, FromVariant)]
enum LispSubExpr<Expr> {
    List(List<Expr>),
    Ident(Ident),
}

#[cgp_new_provider]
impl<Context, Code, Expr, SubExpr, LispExpr, Operator> ComputerRef<Context, Code, SubExpr>
    for BinaryOpToLisp<Operator>
where
    Context: HasExprType<Expr = Expr>
        + HasLispExprType<LispExpr = LispExpr>
        + CanComputeRef<Code, Expr, Output = LispExpr>,
    SubExpr: BinarySubExpression<Expr>,
    Operator: Default + Display,
    LispSubExpr<LispExpr>: CanUpcast<LispExpr>,
{
    type Output = LispExpr;

    fn compute_ref(context: &Context, code: PhantomData<Code>, expr: &SubExpr) -> Self::Output {
        let expr_a = context.compute_ref(code, expr.left());
        let expr_b = context.compute_ref(code, expr.right());

        let ident = LispSubExpr::Ident(Ident(Operator::default().to_string())).upcast(PhantomData);

        LispSubExpr::List(List(vec![ident.into(), expr_a.into(), expr_b.into()]))
            .upcast(PhantomData)
    }
}
