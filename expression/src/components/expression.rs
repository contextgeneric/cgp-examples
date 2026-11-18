use cgp::prelude::*;

/// A context's abstract Lisp-expression type. See
/// [the add_mult example](https://contextgeneric.dev/docs/projects/cgp-examples/expression/examples/add-mult).
#[cgp_type]
pub trait HasLispExprType {
    type LispExpr;
}

/// A context's abstract math-expression type. See
/// [the add_mult example](https://contextgeneric.dev/docs/projects/cgp-examples/expression/examples/add-mult).
#[cgp_type]
pub trait HasMathExprType {
    type MathExpr;
}
