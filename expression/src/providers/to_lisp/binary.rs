use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::vec;
use core::fmt::Display;

use cgp::core::field::impls::CanUpcast;
use cgp::extra::handler::{CanComputeRef, ComputerRef, ComputerRefComponent};
use cgp::prelude::*;

use crate::components::{HasLispExprType, HasMathExprType};
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

#[cgp_impl(new BinaryOpToLisp<Operator>)]
impl<Context, Code, MathExpr, MathSubExpr, LispExpr, Operator> ComputerRef<Code, MathSubExpr>
    for Context
where
    Context: HasMathExprType<MathExpr = MathExpr>
        + HasLispExprType<LispExpr = LispExpr>
        + CanComputeRef<Code, MathExpr, Output = LispExpr>,
    MathSubExpr: BinarySubExpression<MathExpr>,
    Operator: Default + Display,
    LispSubExpr<LispExpr>: CanUpcast<LispExpr>,
{
    type Output = LispExpr;

    fn compute_ref(context: &Context, code: PhantomData<Code>, expr: &MathSubExpr) -> Self::Output {
        let expr_a = context.compute_ref(code, expr.left());
        let expr_b = context.compute_ref(code, expr.right());

        let ident = LispSubExpr::Ident(Ident(Operator::default().to_string())).upcast(PhantomData);

        LispSubExpr::List(List(vec![ident.into(), expr_a.into(), expr_b.into()]))
            .upcast(PhantomData)
    }
}
