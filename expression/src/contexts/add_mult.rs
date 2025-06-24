use cgp::extra::dispatch::{DispatchFields, DispatchFieldsRef};
use cgp::extra::handler::{HandleFieldValue, UseInputDelegate};
use cgp::prelude::*;

use crate::components::LispExprTypeProviderComponent;
use crate::dsl::{Eval, ToLisp};
use crate::providers::{BinaryOpToLisp, EvalAdd, EvalLiteral, EvalMultiply, LiteralToLisp};
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
            UseDelegate<new CodeComponents {
                Eval:
                    UseInputDelegate<
                        new EvalComponents {
                            Expr: DispatchEval,
                            Plus<Expr>: EvalAdd,
                            Times<Expr>: EvalMultiply,
                            Literal<Value>: EvalLiteral,
                        }
                    >,
                ToLisp:
                    UseInputDelegate<
                        new ToLispComponents {
                            <'a> &'a Expr: DispatchEval,
                            <'a> &'a Plus<Expr>: BinaryOpToLisp<symbol!("+")>,
                            <'a> &'a Times<Expr>: BinaryOpToLisp<symbol!("*")>,
                            <'a> &'a Literal<Value>: LiteralToLisp,
                        }
                    >,
            }>
    }
}

#[cgp_new_provider]
impl Computer<Interpreter, Eval, Expr> for DispatchEval {
    type Output = Value;

    fn compute(context: &Interpreter, code: PhantomData<Eval>, expr: Expr) -> Self::Output {
        <DispatchFields<HandleFieldValue<UseContext>>>::compute(context, code, expr)
    }
}

#[cgp_provider]
impl<'a> Computer<Interpreter, ToLisp, &'a Expr> for DispatchEval {
    type Output = LispExpr;

    fn compute(context: &Interpreter, code: PhantomData<ToLisp>, expr: &Expr) -> Self::Output {
        <DispatchFieldsRef<HandleFieldValue<UseContext>>>::compute(context, code, expr)
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

check_components! {
    <'a> CanSerializeToLisp for Interpreter {
        ComputerComponent: [
            (ToLisp, &'a Expr),
            (ToLisp, &'a Literal<Value>),
            (ToLisp, &'a Plus<Expr>),
            (ToLisp, &'a Times<Expr>),
        ]
    }
}

#[cfg(test)]
mod test {
    use core::marker::PhantomData;

    use alloc::borrow::ToOwned;
    use alloc::vec;
    use cgp::extra::handler::CanCompute;

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
            interpreter.compute(
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
            interpreter.compute(
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
            interpreter.compute(
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
