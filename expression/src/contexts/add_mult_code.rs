use cgp::extra::dispatch::MatchWithValueHandlersRef;
use cgp::extra::handler::{ComputerRef, ComputerRefComponent};
use cgp::prelude::*;

use crate::components::{LispExprTypeProviderComponent, MathExprTypeProviderComponent};
use crate::dsl::{Eval, ToLisp};
use crate::providers::{BinaryOpToLisp, EvalAdd, EvalLiteral, EvalMultiply, LiteralToLisp};
use crate::types::{Ident, List, Literal, Plus, Times};

pub type Value = u64;

#[derive(Debug, CgpData)]
pub enum MathExpr {
    Plus(Plus<MathExpr>),
    Times(Times<MathExpr>),
    Literal(Literal<Value>),
}

#[derive(Eq, PartialEq, Debug, CgpData)]
pub enum LispExpr {
    List(List<LispExpr>),
    Literal(Literal<Value>),
    Ident(Ident),
}

pub struct Interpreter;

delegate_components! {
    Interpreter {
        open ComputerRefComponent;

        MathExprTypeProviderComponent:
            UseType<MathExpr>,
        LispExprTypeProviderComponent:
            UseType<LispExpr>,

        @ComputerRefComponent.Eval.MathExpr: DispatchEval,
        @ComputerRefComponent.Eval.Literal<Value>: EvalLiteral,
        @ComputerRefComponent.Eval.Plus<MathExpr>: EvalAdd,
        @ComputerRefComponent.Eval.Times<MathExpr>: EvalMultiply,

        @ComputerRefComponent.ToLisp.MathExpr: DispatchToLisp,
        @ComputerRefComponent.ToLisp.Literal<Value>: LiteralToLisp,
        @ComputerRefComponent.ToLisp.Plus<MathExpr>: BinaryOpToLisp<Symbol!("+")>,
        @ComputerRefComponent.ToLisp.Times<MathExpr>: BinaryOpToLisp<Symbol!("*")>,
    }
}

#[cgp_impl(new DispatchEval)]
impl ComputerRef<Eval, MathExpr> for Interpreter {
    type Output = Value;

    fn compute_ref(
        context: &Interpreter,
        code: PhantomData<Eval>,
        expr: &MathExpr,
    ) -> Self::Output {
        <MatchWithValueHandlersRef>::compute_ref(context, code, expr)
    }
}

#[cgp_impl(new DispatchToLisp)]
impl ComputerRef<ToLisp, MathExpr> for Interpreter {
    type Output = LispExpr;

    fn compute_ref(
        context: &Interpreter,
        code: PhantomData<ToLisp>,
        expr: &MathExpr,
    ) -> Self::Output {
        <MatchWithValueHandlersRef>::compute_ref(context, code, expr)
    }
}

check_components! {
    Interpreter {
        ComputerRefComponent: [
            (Eval, MathExpr),
            (Eval, Literal<Value>),
            (Eval, Plus<MathExpr>),
            (Eval, Times<MathExpr>),
            (ToLisp, MathExpr),
            (ToLisp, Literal<Value>),
            (ToLisp, Plus<MathExpr>),
            (ToLisp, Times<MathExpr>),
        ]
    }
}
