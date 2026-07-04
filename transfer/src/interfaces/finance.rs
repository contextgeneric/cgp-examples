use cgp::prelude::*;

use crate::interfaces::{HasCurrencyType, HasQuantityType, HasUserIdType};

#[cgp_component(UserBalanceQuerier)]
#[async_trait]
#[use_type(HasUserIdType.UserId)]
#[use_type(HasCurrencyType.Currency)]
#[use_type(HasQuantityType.Quantity)]
#[use_type(HasErrorType.Error)]
pub trait CanQueryUserBalance {
    async fn query_user_balance(
        &self,
        user: &UserId,
        currency: &Currency,
    ) -> Result<Quantity, Error>;
}

#[cgp_component(MoneyTransferrer)]
#[async_trait]
#[use_type(HasUserIdType.UserId)]
#[use_type(HasCurrencyType.Currency)]
#[use_type(HasQuantityType.Quantity)]
#[use_type(HasErrorType.Error)]
pub trait CanTransferMoney {
    async fn transfer_money(
        &self,
        sender: &UserId,
        recipient: &UserId,
        currency: &Currency,
        quantity: &Quantity,
    ) -> Result<(), Error>;
}
