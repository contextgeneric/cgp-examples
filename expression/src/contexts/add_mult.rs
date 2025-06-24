use cgp::extra::dispatch::DispatchFields;
use cgp::extra::handler::{HandleFieldValue, UseInputDelegate};
use cgp::prelude::*;

use crate::components::ExpressionTypeProviderComponent;
use crate::dsl::Eval;
use crate::providers::{EvalAdd, EvalLiteral, EvalMultiply};
use crate::types::{Add, Literal, Multiply};

#[derive(Debug, HasFields, FromVariant, ExtractField)]
pub enum Expr {
    Add(Add<Expr>),
    Multiply(Multiply<Expr>),
    Literal(Literal<u64>),
}

#[cgp_context]
pub struct Interpreter;

delegate_components! {
    InterpreterComponents {
        ExpressionTypeProviderComponent:
            UseType<Expr>,
        ComputerComponent:
            UseDelegate<new CodeComponents {
                Eval: UseInputDelegate<EvalComponents>,
            }>
    }
}

delegate_components! {
    new EvalComponents {
        Expr: DispatchExpr,
        Add<Expr>: EvalAdd,
        Multiply<Expr>: EvalMultiply,
        Literal<u64>: EvalLiteral,
    }
}

#[cgp_new_provider]
impl Computer<Interpreter, Eval, Expr> for DispatchExpr {
    type Output = u64;

    fn compute(context: &Interpreter, code: PhantomData<Eval>, expr: Expr) -> Self::Output {
        <DispatchFields<HandleFieldValue<UseContext>>>::compute(context, code, expr)
    }
}

check_components! {
    CanUseInterpreter for Interpreter {
        ComputerComponent: [
            (Eval, Expr),
            (Eval, Literal<u64>),
            (Eval, Add<Expr>),
        ]
    }
}
