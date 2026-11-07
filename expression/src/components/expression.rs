use cgp::prelude::*;

/// A context's abstract Lisp-expression type.
#[cgp_type]
pub trait HasLispExprType {
    type LispExpr;
}

/// A context's abstract math-expression type.
#[cgp_type]
pub trait HasMathExprType {
    type MathExpr;
}
