use cgp::extra::dispatch::MatchWithValueHandlersRef;
use cgp::extra::handler::{ComputerRef, ComputerRefComponent, UseInputDelegate};
use cgp::prelude::*;

use crate::dsl::Eval;
use crate::providers::{EvalAdd, EvalLiteral, EvalMultiply, EvalNegate, EvalSubtract};
use crate::types::{Literal, Minus, Negate, Plus, Times};

pub type Value = i64;

#[derive(Debug, HasFields, FromVariant, ExtractField)]
pub enum MathPlusExpr {
    Plus(Plus<MathPlusExpr>),
    Times(Times<MathPlusExpr>),
    Literal(Literal<Value>),
    Negate(Negate<MathPlusExpr>),
    Minus(Minus<MathPlusExpr>),
}

pub struct InterpreterPlus;

delegate_components! {
    InterpreterPlus {
        ComputerRefComponent:
            UseDelegate<new CodeComponents {
                Eval: UseInputDelegate<new EvalComponents {
                    MathPlusExpr: DispatchEval,
                    Plus<MathPlusExpr>: EvalAdd,
                    Times<MathPlusExpr>: EvalMultiply,
                    Literal<Value>: EvalLiteral,
                    Minus<MathPlusExpr>: EvalSubtract,
                    Negate<MathPlusExpr>: EvalNegate,
                }>,
            }>
    }
}

#[cgp_impl(new DispatchEval)]
impl ComputerRef<Eval, MathPlusExpr> for InterpreterPlus {
    type Output = Value;

    fn compute_ref(
        context: &InterpreterPlus,
        code: PhantomData<Eval>,
        expr: &MathPlusExpr,
    ) -> Self::Output {
        <MatchWithValueHandlersRef>::compute_ref(context, code, expr)
    }
}

check_components! {
    CanUseInterpreter for InterpreterPlus {
        ComputerRefComponent: [
            (Eval, MathPlusExpr),
            (Eval, Literal<Value>),
            (Eval, Plus<MathPlusExpr>),
            (Eval, Negate<MathPlusExpr>),
            (Eval, Minus<MathPlusExpr>),
        ]
    }
}

#[cfg(test)]
mod test {
    use core::marker::PhantomData;

    use cgp::extra::handler::CanComputeRef;

    use crate::contexts::add_mult_neg::{InterpreterPlus, MathPlusExpr};
    use crate::dsl::Eval;
    use crate::types::{Literal, Minus, Negate, Plus, Times};

    #[test]
    fn test_add_mult_neg() {
        let interpreter = InterpreterPlus;
        let code = PhantomData::<Eval>;

        assert_eq!(
            interpreter.compute_ref(
                code,
                &MathPlusExpr::Plus(Plus {
                    left: MathPlusExpr::Literal(Literal(2)).into(),
                    right: MathPlusExpr::Literal(Literal(3)).into(),
                })
            ),
            5,
        );

        assert_eq!(
            interpreter.compute_ref(
                code,
                &MathPlusExpr::Times(Times {
                    left: MathPlusExpr::Literal(Literal(2)).into(),
                    right: MathPlusExpr::Literal(Literal(3)).into(),
                })
            ),
            6,
        );

        assert_eq!(
            interpreter.compute_ref(
                code,
                &MathPlusExpr::Minus(Minus {
                    left: MathPlusExpr::Literal(Literal(2)).into(),
                    right: MathPlusExpr::Literal(Literal(3)).into(),
                })
            ),
            -1,
        );

        assert_eq!(
            interpreter.compute_ref(
                code,
                &MathPlusExpr::Times(Times {
                    left: MathPlusExpr::Negate(Negate(MathPlusExpr::Literal(Literal(2)).into()))
                        .into(),
                    right: MathPlusExpr::Plus(Plus {
                        left: MathPlusExpr::Literal(Literal(3)).into(),
                        right: MathPlusExpr::Literal(Literal(4)).into(),
                    })
                    .into(),
                })
            ),
            -14,
        );
    }
}
