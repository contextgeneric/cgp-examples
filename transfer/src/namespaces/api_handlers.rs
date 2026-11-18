use cgp::prelude::*;

use crate::interfaces::{QueryBalanceApi, TransferApi};
use crate::providers::{
    HandleFromRequest, HandleQueryBalance, HandleTransfer, ResponseToJson, UseBasicAuth,
};
use crate::types::{
    AxumQueryBalanceRequest, AxumTransferRequest, QueryBalanceRequest, TransferRequest,
};

// The application's API surface: which handler pipeline serves each API. This is
// a plain lookup table keyed by the `Api` marker rather than a namespace a
// context joins, so a context pulls it in with a `for <Key, Value> in
// DefaultApiHandlers` loop and maps each entry onto its own `ApiHandlerComponent`
// dispatch path.
cgp_namespace! {
    new DefaultApiHandlers {
        QueryBalanceApi:
            HandleFromRequest<
                AxumQueryBalanceRequest,
                ResponseToJson<UseBasicAuth<HandleQueryBalance<QueryBalanceRequest>>>,
            >,
        TransferApi:
            HandleFromRequest<
                AxumTransferRequest,
                UseBasicAuth<HandleTransfer<TransferRequest>>,
            >,
    }
}
