use cgp::extra::dispatch::{MatchWithValueHandlers, MatchWithValueHandlersRef};
use cgp::extra::handler::{ComputerRef, ComputerRefComponent};
use cgp::prelude::*;

use crate::components::{LispExprTypeProviderComponent, MathExprTypeProviderComponent};
use crate::dsl::{Eval, ToLisp};
use crate::providers::{
    EvalAdd, EvalLiteral, EvalMultiply, LiteralToLisp, PlusToLisp, TimesToLisp,
};
use crate::types::{Ident, List, Literal, Plus, Times};

pub type Value = u64;

#[derive(Debug, CgpData)]
pub enum MathExpr {
    Plus(Plus<MathExpr>),
    Times(Times<MathExpr>),
    Literal(Literal<Value>),
}

#[derive(Eq, PartialEq, Debug, CgpData)]
pub enum LispExpr {
    List(List<LispExpr>),
    Literal(Literal<Value>),
    Ident(Ident),
}

pub struct Interpreter;

delegate_components! {
    Interpreter {
        open { ComputerComponent, ComputerRefComponent };

        MathExprTypeProviderComponent:
            UseType<MathExpr>,
        LispExprTypeProviderComponent:
            UseType<LispExpr>,

        @ComputerComponent.<Code> Code.MathExpr: DispatchEval,
        @ComputerComponent.<Code> Code.Plus<MathExpr>: EvalAdd,
        @ComputerComponent.<Code> Code.Times<MathExpr>: EvalMultiply,
        @ComputerComponent.<Code> Code.Literal<Value>: EvalLiteral,

        @ComputerRefComponent.<Code> Code.MathExpr: DispatchToLisp,
        @ComputerRefComponent.<Code> Code.Literal<Value>: LiteralToLisp,
        @ComputerRefComponent.<Code> Code.Plus<MathExpr>: PlusToLisp,
        @ComputerRefComponent.<Code> Code.Times<MathExpr>: TimesToLisp,
    }
}

#[cgp_impl(new DispatchEval)]
impl<Code> Computer<Code, MathExpr> for Interpreter {
    type Output = Value;

    fn compute(context: &Interpreter, code: PhantomData<Code>, expr: MathExpr) -> Self::Output {
        <MatchWithValueHandlers>::compute(context, code, expr)
    }
}

#[cgp_impl(new DispatchToLisp)]
impl<Code> ComputerRef<Code, MathExpr> for Interpreter {
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
    Interpreter {
        ComputerComponent: [
            (Eval, MathExpr),
            (Eval, Literal<Value>),
            (Eval, Plus<MathExpr>),
            (Eval, Times<MathExpr>),
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
    use alloc::borrow::ToOwned;
    use alloc::vec;
    use core::marker::PhantomData;

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
