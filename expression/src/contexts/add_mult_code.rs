use cgp::extra::dispatch::MatchWithValueHandlersRef;
use cgp::extra::handler::{ComputerRef, ComputerRefComponent, UseInputDelegate};
use cgp::prelude::*;

use crate::components::{LispExprTypeProviderComponent, MathExprTypeProviderComponent};
use crate::dsl::{Eval, ToLisp};
use crate::providers::{BinaryOpToLisp, EvalAdd, EvalLiteral, EvalMultiply, LiteralToLisp};
use crate::types::{Ident, List, Literal, Plus, Times};

pub type Value = u64;

#[derive(Debug, HasFields, FromVariant, ExtractField)]
pub enum MathExpr {
    Plus(Plus<MathExpr>),
    Times(Times<MathExpr>),
    Literal(Literal<Value>),
}

#[derive(Eq, PartialEq, Debug, HasFields, FromVariant, ExtractField)]
pub enum LispExpr {
    List(List<LispExpr>),
    Literal(Literal<Value>),
    Ident(Ident),
}

#[cgp_context]
pub struct Interpreter;

delegate_components! {
    InterpreterComponents {
        MathExprTypeProviderComponent:
            UseType<MathExpr>,
        LispExprTypeProviderComponent:
            UseType<LispExpr>,
        ComputerRefComponent:
            UseInputDelegate<
                new ExprComputerComponents {
                    MathExpr: HandleMathExpr,
                    Literal<Value>: HandleLiteral,
                    Plus<MathExpr>: HandlePlus,
                    Times<MathExpr>: HandleTimes,
                }
            >,
    }
}

delegate_components! {
    new HandlePlus {
        ComputerRefComponent: UseDelegate<
            new PlusHandlers {
                Eval: EvalAdd,
                ToLisp: BinaryOpToLisp<Symbol!("+")>,
            }>
    }
}

delegate_components! {
    new HandleTimes {
        ComputerRefComponent: UseDelegate<
            new TimesHandlers {
                Eval: EvalMultiply,
                ToLisp: BinaryOpToLisp<Symbol!("*")>,
            }>
    }
}

delegate_components! {
    new HandleLiteral {
        ComputerRefComponent: UseDelegate<
            new LiteralHandlers {
                Eval: EvalLiteral,
                ToLisp: LiteralToLisp,
            }>
    }
}

delegate_components! {
    new HandleMathExpr {
        ComputerRefComponent: UseDelegate<
            new MathExprHandlers {
                Eval: DispatchEval,
                ToLisp: DispatchToLisp,
            }>
    }
}

#[cgp_new_provider]
impl ComputerRef<Interpreter, Eval, MathExpr> for DispatchEval {
    type Output = Value;

    fn compute_ref(
        context: &Interpreter,
        code: PhantomData<Eval>,
        expr: &MathExpr,
    ) -> Self::Output {
        <MatchWithValueHandlersRef>::compute_ref(context, code, expr)
    }
}

#[cgp_new_provider]
impl ComputerRef<Interpreter, ToLisp, MathExpr> for DispatchToLisp {
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
    CanUseInterpreter for Interpreter {
        ComputerRefComponent: [
            (Eval, MathExpr),
            (Eval, Literal<Value>),
            (Eval, Plus<MathExpr>),
            (ToLisp, MathExpr),
            (ToLisp, Literal<Value>),
            (ToLisp, Plus<MathExpr>),
            (ToLisp, Times<MathExpr>),
        ]
    }
}
