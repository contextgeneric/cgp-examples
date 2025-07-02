use cgp::extra::dispatch::{MatchWithValueHandlers, MatchWithValueHandlersRef};
use cgp::extra::handler::{ComputerRef, ComputerRefComponent, UseInputDelegate};
use cgp::prelude::*;

use crate::components::{ExprTypeProviderComponent, LispExprTypeProviderComponent};
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
        ExprTypeProviderComponent:
            UseType<MathExpr>,
        LispExprTypeProviderComponent:
            UseType<LispExpr>,
        ComputerComponent:
            UseInputDelegate<
                new EvalComponents {
                    MathExpr: DispatchEval,
                    Plus<MathExpr>: EvalAdd,
                    Times<MathExpr>: EvalMultiply,
                    Literal<Value>: EvalLiteral,
                }
            >,
        ComputerRefComponent:
            UseInputDelegate<
                new ToLispComponents {
                    MathExpr: DispatchToLisp,
                    Literal<Value>: LiteralToLisp,
                    Plus<MathExpr>: BinaryOpToLisp<symbol!("+")>,
                    Times<MathExpr>: BinaryOpToLisp<symbol!("*")>,
                    // Plus<Expr>: PlusToLisp,
                    // Times<Expr>: TimesToLisp,
                }
            >,
    }
}

#[cgp_new_provider]
impl<Code> Computer<Interpreter, Code, MathExpr> for DispatchEval {
    type Output = Value;

    fn compute(context: &Interpreter, code: PhantomData<Code>, expr: MathExpr) -> Self::Output {
        MatchWithValueHandlers::compute(context, code, expr)
    }
}

#[cgp_new_provider]
impl<Code> ComputerRef<Interpreter, Code, MathExpr> for DispatchToLisp {
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
    CanUseInterpreter for Interpreter {
        ComputerComponent: [
            (Eval, MathExpr),
            (Eval, Literal<Value>),
            (Eval, Plus<MathExpr>),
        ],
        ComputerRefComponent: [
            (ToLisp, MathExpr),
            (ToLisp, Literal<Value>),
            (ToLisp, Plus<MathExpr>),
            (ToLisp, Times<MathExpr>),
        ]
    }
}

#[cfg(test)]
mod test {
    use core::marker::PhantomData;

    use alloc::borrow::ToOwned;
    use alloc::vec;
    use cgp::extra::handler::{CanCompute, CanComputeRef};

    use crate::contexts::add_mult::{Interpreter, LispExpr, MathExpr};
    use crate::dsl::{Eval, ToLisp};
    use crate::types::{Ident, List, Literal, Plus, Times};

    #[test]
    fn test_add_mult() {
        let interpreter = Interpreter;
        let code = PhantomData::<Eval>;

        assert_eq!(
            interpreter.compute(
                code,
                MathExpr::Plus(Plus {
                    left: MathExpr::Literal(Literal(2)).into(),
                    right: MathExpr::Literal(Literal(3)).into()
                })
            ),
            5,
        );

        assert_eq!(
            interpreter.compute(
                code,
                MathExpr::Times(Times {
                    left: MathExpr::Literal(Literal(2)).into(),
                    right: MathExpr::Literal(Literal(3)).into()
                })
            ),
            6,
        );

        assert_eq!(
            interpreter.compute(
                code,
                MathExpr::Times(Times {
                    left: MathExpr::Literal(Literal(2)).into(),
                    right: MathExpr::Plus(Plus {
                        left: MathExpr::Literal(Literal(3)).into(),
                        right: MathExpr::Literal(Literal(4)).into()
                    })
                    .into(),
                })
            ),
            14,
        );
    }

    #[test]
    fn test_add_mult_to_lisp() {
        let interpreter = Interpreter;
        let code = PhantomData::<ToLisp>;

        assert_eq!(
            interpreter.compute_ref(
                code,
                &MathExpr::Plus(Plus {
                    left: MathExpr::Literal(Literal(2)).into(),
                    right: MathExpr::Literal(Literal(3)).into()
                })
            ),
            LispExpr::List(List(vec![
                LispExpr::Ident(Ident("+".to_owned())).into(),
                LispExpr::Literal(Literal(2)).into(),
                LispExpr::Literal(Literal(3)).into()
            ]))
        );

        assert_eq!(
            interpreter.compute_ref(
                code,
                &MathExpr::Times(Times {
                    left: MathExpr::Literal(Literal(2)).into(),
                    right: MathExpr::Literal(Literal(3)).into()
                })
            ),
            LispExpr::List(List(vec![
                LispExpr::Ident(Ident("*".to_owned())).into(),
                LispExpr::Literal(Literal(2)).into(),
                LispExpr::Literal(Literal(3)).into()
            ]))
        );

        assert_eq!(
            interpreter.compute_ref(
                code,
                &MathExpr::Times(Times {
                    left: MathExpr::Literal(Literal(2)).into(),
                    right: MathExpr::Plus(Plus {
                        left: MathExpr::Literal(Literal(3)).into(),
                        right: MathExpr::Literal(Literal(4)).into()
                    })
                    .into(),
                })
            ),
            LispExpr::List(List(vec![
                LispExpr::Ident(Ident("*".to_owned())).into(),
                LispExpr::Literal(Literal(2)).into(),
                LispExpr::List(List(vec![
                    LispExpr::Ident(Ident("+".to_owned())).into(),
                    LispExpr::Literal(Literal(3)).into(),
                    LispExpr::Literal(Literal(4)).into()
                ]))
                .into(),
            ]))
        );
    }
}
