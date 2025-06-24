use cgp::extra::dispatch::DispatchFields;
use cgp::extra::handler::{HandleFieldValue, UseInputDelegate};
use cgp::prelude::*;

use crate::components::ExpressionTypeProviderComponent;
use crate::dsl::Eval;
use crate::providers::{EvalAdd, EvalLiteral, EvalMultiply, EvalNegate};
use crate::types::{Add, Literal, Multiply, Negate};

pub type Value = i64;

#[derive(Debug, HasFields, FromVariant, ExtractField)]
pub enum Expr {
    Add(Add<Expr>),
    Multiply(Multiply<Expr>),
    Literal(Literal<Value>),
    Negate(Negate<Expr>),
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
        Literal<Value>: EvalLiteral,
        Negate<Expr>: EvalNegate,
    }
}

#[cgp_new_provider]
impl Computer<Interpreter, Eval, Expr> for DispatchExpr {
    type Output = Value;

    fn compute(context: &Interpreter, code: PhantomData<Eval>, expr: Expr) -> Self::Output {
        <DispatchFields<HandleFieldValue<UseContext>>>::compute(context, code, expr)
    }
}

check_components! {
    CanUseInterpreter for Interpreter {
        ComputerComponent: [
            (Eval, Expr),
            (Eval, Literal<Value>),
            (Eval, Add<Expr>),
            (Eval, Negate<Expr>),
        ]
    }
}
