use cgp::extra::dispatch::MatchWithValueHandlersRef;
use cgp::extra::handler::{ComputerRef, ComputerRefComponent, UseInputDelegate};
use cgp::prelude::*;

use crate::dsl::Eval;
use crate::providers::{EvalAdd, EvalLiteral, EvalMultiply, EvalNegate, EvalSubtract};
use crate::types::{Literal, Minus, Negate, Plus, Times};

pub type Value = i64;

#[derive(Debug, HasFields, FromVariant, ExtractField)]
pub enum MathExpr {
    Plus(Plus<MathExpr>),
    Times(Times<MathExpr>),
    Literal(Literal<Value>),
    Negate(Negate<MathExpr>),
    Minus(Minus<MathExpr>),
}

#[cgp_context]
pub struct Interpreter;

delegate_components! {
    InterpreterComponents {
        ComputerRefComponent:
            UseDelegate<new CodeComponents {
                Eval: UseInputDelegate<EvalComponents>,
            }>
    }
}

delegate_components! {
    new EvalComponents {
        MathExpr: DispatchExpr,
        Plus<MathExpr>: EvalAdd,
        Times<MathExpr>: EvalMultiply,
        Literal<Value>: EvalLiteral,
        Negate<MathExpr>: EvalNegate,
        Minus<MathExpr>: EvalSubtract,
    }
}

#[cgp_new_provider]
impl ComputerRef<Interpreter, Eval, MathExpr> for DispatchExpr {
    type Output = Value;

    fn compute_ref(
        context: &Interpreter,
        code: PhantomData<Eval>,
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
            (Eval, Negate<MathExpr>),
            (Eval, Minus<MathExpr>),
        ]
    }
}

#[cfg(test)]
mod test {
    use core::marker::PhantomData;

    use cgp::extra::handler::CanComputeRef;

    use crate::contexts::add_mult_neg::{Interpreter, MathExpr};
    use crate::dsl::Eval;
    use crate::types::{Literal, Minus, Negate, Plus, Times};

    #[test]
    fn test_add_mult_neg() {
        let interpreter = Interpreter;
        let code = PhantomData::<Eval>;

        assert_eq!(
            interpreter.compute_ref(
                code,
                &MathExpr::Plus(Plus {
                    left: MathExpr::Literal(Literal(2)).into(),
                    right: MathExpr::Literal(Literal(3)).into(),
                })
            ),
            5,
        );

        assert_eq!(
            interpreter.compute_ref(
                code,
                &MathExpr::Times(Times {
                    left: MathExpr::Literal(Literal(2)).into(),
                    right: MathExpr::Literal(Literal(3)).into(),
                })
            ),
            6,
        );

        assert_eq!(
            interpreter.compute_ref(
                code,
                &MathExpr::Minus(Minus {
                    left: MathExpr::Literal(Literal(2)).into(),
                    right: MathExpr::Literal(Literal(3)).into(),
                })
            ),
            -1,
        );

        assert_eq!(
            interpreter.compute_ref(
                code,
                &MathExpr::Times(Times {
                    left: MathExpr::Negate(Negate(MathExpr::Literal(Literal(2)).into())).into(),
                    right: MathExpr::Plus(Plus {
                        left: MathExpr::Literal(Literal(3)).into(),
                        right: MathExpr::Literal(Literal(4)).into(),
                    })
                    .into(),
                })
            ),
            -14,
        );
    }
}
