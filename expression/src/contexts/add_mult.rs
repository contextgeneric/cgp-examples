use std::process::Output;

use cgp::extra::dispatch::DispatchFields;
use cgp::extra::handler::{CanCompute, HandleFieldValue, UseInputDelegate};
use cgp::prelude::*;

use crate::components::ExpressionTypeProviderComponent;
use crate::dsl::Eval;
use crate::providers::{EvalAdd, EvalLiteral, EvalMultiply};
use crate::types::{Add, Literal, Multiply};

#[derive(Debug, HasFields, FromVariant, ExtractField)]
pub enum Expr {
    Add(Add<Expr>),
    // Multiply(Multiply<Expr>),
    Literal(Literal<u64>),
}

#[cgp_context]
pub struct Interpreter;

delegate_components! {
    InterpreterComponents {
        ExpressionTypeProviderComponent:
            UseType<Expr>,
        ComputerComponent:
            UseInputDelegate<EvalComponents>,
            // UseDelegate<new CodeComponents {
            //     Eval: UseInputDelegate<EvalComponents>,
            // }>
    }
}

delegate_components! {
    new EvalComponents {
        // Expr: DispatchFields<HandleFieldValue<UseContext>>,
        Expr: EvalExpr,
        Add<Expr>: EvalAdd,
        Multiply<Expr>: EvalMultiply,
        Literal<u64>: EvalLiteral,
    }
}

#[cgp_new_provider]
impl<Context, Code> Computer<Context, Code, Expr> for EvalExpr
where
    Context: CanCompute<Code, Add<Expr>>,
{
    type Output = u64;

    fn compute(context: &Context, code: PhantomData<Code>, expr: Expr) -> Self::Output {
        todo!()
    }
}

// check_components! {
//     CanUseInterpreter for Interpreter {
//         ComputerComponent: [
//             (Eval, Expr),
//             (Eval, Literal<u64>),
//             // (Eval, Add<Expr>),
//         ]
//     }
// }

pub trait CheckContext: CanCompute<Eval, Expr> {}

impl CheckContext for Interpreter {}

// pub trait CheckProvider: Computer<Interpreter, Eval, Expr, Output = u64> {}

// impl CheckProvider for DispatchFields<HandleFieldValue<UseContext>> {}
