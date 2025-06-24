use cgp::prelude::*;

#[cgp_type]
pub trait HasLispExprType {
    type LispExpr;
}
