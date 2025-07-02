use cgp::extra::dispatch::{MatchWithValueHandlers, MatchWithValueHandlersRef};
use cgp::extra::handler::{ComputerRef, ComputerRefComponent, UseInputDelegate};
use cgp::prelude::*;

use crate::components::LispExprTypeProviderComponent;
use crate::dsl::{Eval, ToLisp};
use crate::providers::{
    EvalAdd, EvalLiteral, EvalMultiply, LiteralToLisp, PlusToLisp, TimesToLisp,
};
use crate::types::{Ident, List, Literal, Plus, Times};

pub type Value = u64;

#[derive(Debug, HasFields, FromVariant, ExtractField)]
pub enum Expr {
    Plus(Plus<Expr>),
    Times(Times<Expr>),
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
        LispExprTypeProviderComponent:
            UseType<LispExpr>,
        ComputerComponent:
            UseInputDelegate<
                new EvalComponents {
                    Expr: DispatchEval,
                    Plus<Expr>: EvalAdd,
                    Times<Expr>: EvalMultiply,
                    Literal<Value>: EvalLiteral,
                }
            >,
        ComputerRefComponent:
            UseInputDelegate<
                new ToLispComponents {
                    Expr: DispatchToLisp,
                    Literal<Value>: LiteralToLisp,
                    // Plus<Expr>: BinaryOpToLisp<symbol!("+")>,
                    // Times<Expr>: BinaryOpToLisp<symbol!("*")>,
                    Plus<Expr>: PlusToLisp,
                    Times<Expr>: TimesToLisp,
                }
            >,
    }
}

#[cgp_new_provider]
impl<Code> Computer<Interpreter, Code, Expr> for DispatchEval {
    type Output = Value;

    fn compute(context: &Interpreter, code: PhantomData<Code>, expr: Expr) -> Self::Output {
        MatchWithValueHandlers::compute(context, code, expr)
    }
}

#[cgp_new_provider]
impl<Code> ComputerRef<Interpreter, Code, Expr> for DispatchToLisp {
    type Output = LispExpr;

    fn compute_ref(context: &Interpreter, code: PhantomData<Code>, expr: &Expr) -> Self::Output {
        <MatchWithValueHandlersRef>::compute_ref(context, code, expr)
    }
}

check_components! {
    CanUseInterpreter for Interpreter {
        ComputerComponent: [
            (Eval, Expr),
            (Eval, Literal<Value>),
            (Eval, Plus<Expr>),
        ],
        ComputerRefComponent: [
            (ToLisp, Expr),
            (ToLisp, Literal<Value>),
            (ToLisp, Plus<Expr>),
            (ToLisp, Times<Expr>),
        ]
    }
}

#[cfg(test)]
mod test {
    use core::marker::PhantomData;

    use alloc::borrow::ToOwned;
    use alloc::vec;
    use cgp::extra::handler::{CanCompute, CanComputeRef};

    use crate::contexts::add_mult::{Expr, Interpreter, LispExpr};
    use crate::dsl::{Eval, ToLisp};
    use crate::types::{Ident, List, Literal, Plus, Times};

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

    #[test]
    fn test_add_mult_to_lisp() {
        let interpreter = Interpreter;
        let code = PhantomData::<ToLisp>;

        assert_eq!(
            interpreter.compute_ref(
                code,
                &Expr::Plus(Plus(
                    Expr::Literal(Literal(2)).into(),
                    Expr::Literal(Literal(3)).into()
                ))
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
                &Expr::Times(Times(
                    Expr::Literal(Literal(2)).into(),
                    Expr::Literal(Literal(3)).into()
                ))
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
                &Expr::Times(Times(
                    Expr::Literal(Literal(2)).into(),
                    Expr::Plus(Plus(
                        Expr::Literal(Literal(3)).into(),
                        Expr::Literal(Literal(4)).into()
                    ))
                    .into(),
                ))
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
