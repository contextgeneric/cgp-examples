use cgp::extra::dispatch::DispatchFields;
use cgp::extra::handler::{HandleFieldValue, UseInputDelegate};
use cgp::prelude::*;

use crate::dsl::Eval;
use crate::providers::{EvalAdd, EvalLiteral, EvalMultiply};
use crate::types::{Literal, Plus, Times};

pub type Value = u64;

#[derive(Debug, HasFields, FromVariant, ExtractField)]
pub enum Expr {
    Plus(Plus<Expr>),
    Times(Times<Expr>),
    Literal(Literal<Value>),
}

#[cgp_context]
pub struct Interpreter;

delegate_components! {
    InterpreterComponents {
        ComputerComponent:
            UseDelegate<new CodeComponents {
                Eval: UseInputDelegate<EvalComponents>,
            }>
    }
}

delegate_components! {
    new EvalComponents {
        Expr: DispatchExpr,
        Plus<Expr>: EvalAdd,
        Times<Expr>: EvalMultiply,
        Literal<Value>: EvalLiteral,
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
            (Eval, Plus<Expr>),
        ]
    }
}

#[cfg(test)]
mod test {
    use core::marker::PhantomData;

    use cgp::extra::handler::CanCompute;

    use crate::contexts::add_mult::{Expr, Interpreter};
    use crate::dsl::Eval;
    use crate::types::{Literal, Plus, Times};

    #[test]
    fn test_add_mult() {
        let interpreter = Interpreter;
        let code = PhantomData::<Eval>;

        assert_eq!(
            interpreter.compute(
                code,
                Expr::Plus(Plus(
                    Expr::Literal(Literal(2)).into(),
                    Expr::Literal(Literal(3)).into()
                ))
            ),
            5,
        );

        assert_eq!(
            interpreter.compute(
                code,
                Expr::Times(Times(
                    Expr::Literal(Literal(2)).into(),
                    Expr::Literal(Literal(3)).into()
                ))
            ),
            6,
        );

        assert_eq!(
            interpreter.compute(
                code,
                Expr::Times(Times(
                    Expr::Literal(Literal(2)).into(),
                    Expr::Plus(Plus(
                        Expr::Literal(Literal(3)).into(),
                        Expr::Literal(Literal(4)).into()
                    ))
                    .into(),
                ))
            ),
            14,
        );
    }
}
