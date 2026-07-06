use cgp::prelude::*;

use crate::interfaces::{HasCurrencyType, HasQuantityType, HasUserIdType};

// The two business capabilities the endpoints call. Each is a component abstract over the
// backend, so the balance-query handler and transfer handler depend only on these traits and
// never on the concrete store behind them. `#[use_type]` imports the domain's abstract types
// so the signatures read in plain names.

// Capability to read a user's balance in a currency.
#[cgp_component(UserBalanceQuerier)]
#[prefix(@app.finance in DefaultNamespace)]
#[async_trait]
#[use_type(HasUserIdType.UserId, HasCurrencyType.Currency, HasQuantityType.Quantity, HasErrorType.Error)]
pub trait CanQueryUserBalance {
    async fn query_user_balance(
        &self,
        user: &UserId,
        currency: &Currency,
    ) -> Result<Quantity, Error>;
}

// Capability to move a quantity of a currency from one user to another.
#[cgp_component(MoneyTransferrer)]
#[prefix(@app.finance in DefaultNamespace)]
#[async_trait]
#[use_type(HasUserIdType.UserId, HasCurrencyType.Currency, HasQuantityType.Quantity, HasErrorType.Error)]
pub trait CanTransferMoney {
    async fn transfer_money(
        &self,
        sender: &UserId,
        recipient: &UserId,
        currency: &Currency,
        quantity: &Quantity,
    ) -> Result<(), Error>;
}
