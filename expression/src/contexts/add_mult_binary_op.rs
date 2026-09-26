use cgp::extra::dispatch::{MatchWithValueHandlers, MatchWithValueHandlersRef};
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
        open { ComputerComponent, ComputerRefComponent };

        MathExprTypeProviderComponent:
            UseType<MathExpr>,
        LispExprTypeProviderComponent:
            UseType<LispExpr>,

        @ComputerComponent.<Code> Code.MathExpr: DispatchEval,
        @ComputerComponent.<Code> Code.Plus<MathExpr>: EvalAdd,
        @ComputerComponent.<Code> Code.Times<MathExpr>: EvalMultiply,
        @ComputerComponent.<Code> Code.Literal<Value>: EvalLiteral,

        @ComputerRefComponent.<Code> Code.MathExpr: DispatchToLisp,
        @ComputerRefComponent.<Code> Code.Literal<Value>: LiteralToLisp,
        @ComputerRefComponent.<Code> Code.Plus<MathExpr>: BinaryOpToLisp<Symbol!("+")>,
        @ComputerRefComponent.<Code> Code.Times<MathExpr>: BinaryOpToLisp<Symbol!("*")>,
    }
}

#[cgp_impl(new DispatchEval)]
impl<Code> Computer<Code, MathExpr> for Interpreter {
    type Output = Value;

    fn compute(context: &Interpreter, code: PhantomData<Code>, expr: MathExpr) -> Self::Output {
        <MatchWithValueHandlers>::compute(context, code, expr)
    }
}

#[cgp_impl(new DispatchToLisp)]
impl<Code> ComputerRef<Code, MathExpr> for Interpreter {
    type Output = LispExpr;

    fn compute_ref(
        context: &Interpreter,
        code: PhantomData<Code>,
        expr: &MathExpr,
    ) -> Self::Output {
        <MatchWithValueHandlersRef>::compute_ref(context, code, expr)
    }
}

check_components! {
    Interpreter {
        ComputerComponent: [
            (Eval, MathExpr),
            (Eval, Literal<Value>),
            (Eval, Plus<MathExpr>),
            (Eval, Times<MathExpr>),
        ],
        ComputerRefComponent: [
            (ToLisp, MathExpr),
            (ToLisp, Literal<Value>),
            (ToLisp, Plus<MathExpr>),
            (ToLisp, Times<MathExpr>),
        ]
    }
}
