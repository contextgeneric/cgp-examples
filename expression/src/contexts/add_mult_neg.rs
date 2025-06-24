use cgp::extra::dispatch::DispatchFields;
use cgp::extra::handler::{HandleFieldValue, UseInputDelegate};
use cgp::prelude::*;

use crate::components::ExpressionTypeProviderComponent;
use crate::dsl::Eval;
use crate::providers::{EvalAdd, EvalLiteral, EvalMultiply, EvalNegate, EvalSubtractWithNegate};
use crate::types::{Literal, Minus, Negate, Plus, Times};

pub type Value = i64;

#[derive(Debug, HasFields, FromVariant, ExtractField)]
pub enum Expr {
    Plus(Plus<Expr>),
    Times(Times<Expr>),
    Literal(Literal<Value>),
    Negate(Negate<Expr>),
    Minus(Minus<Expr>),
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
        Plus<Expr>: EvalAdd,
        Times<Expr>: EvalMultiply,
        Literal<Value>: EvalLiteral,
        Negate<Expr>: EvalNegate,
        Minus<Expr>: EvalSubtractWithNegate,
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
            (Eval, Negate<Expr>),
        ]
    }
}

#[cfg(test)]
mod test {
    use core::marker::PhantomData;

    use cgp::extra::handler::CanCompute;

    use crate::contexts::add_mult_neg::{Expr, Interpreter};
    use crate::dsl::Eval;
    use crate::types::{Literal, Minus, Negate, Plus, Times};

    #[test]
    fn test_add_mult_neg() {
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
                Expr::Minus(Minus(
                    Expr::Literal(Literal(2)).into(),
                    Expr::Literal(Literal(3)).into()
                ))
            ),
            -1,
        );

        assert_eq!(
            interpreter.compute(
                code,
                Expr::Times(Times(
                    Expr::Negate(Negate(Expr::Literal(Literal(2)).into())).into(),
                    Expr::Plus(Plus(
                        Expr::Literal(Literal(3)).into(),
                        Expr::Literal(Literal(4)).into()
                    ))
                    .into(),
                ))
            ),
            -14,
        );
    }
}
