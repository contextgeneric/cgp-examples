use cgp::prelude::*;

#[cgp_type]
pub trait HasLispExprType {
    type LispExpr;
}

#[cgp_type]
pub trait HasExprType {
    type Expr;
}
