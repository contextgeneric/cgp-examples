use cgp::prelude::*;

use crate::interfaces::{HasCurrencyType, HasQuantityType, HasUserIdType};

#[cgp_component(UserBalanceQuerier)]
#[async_trait]
pub trait CanQueryUserBalance:
    HasUserIdType + HasCurrencyType + HasQuantityType + HasErrorType
{
    async fn query_user_balance(
        &self,
        user: &Self::UserId,
        currency: &Self::Currency,
    ) -> Result<Self::Quantity, Self::Error>;
}

#[cgp_component(MoneyTransferrer)]
#[async_trait]
pub trait CanTransferMoney:
    HasUserIdType + HasCurrencyType + HasQuantityType + HasErrorType
{
    async fn transfer_money(
        &self,
        sender: &Self::UserId,
        recipient: &Self::UserId,
        currency: &Self::Currency,
        quantity: &Self::Quantity,
    ) -> Result<(), Self::Error>;
}
