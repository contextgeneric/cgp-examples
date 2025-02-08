use cgp::prelude::*;

use crate::interfaces::{HasCurrencyType, HasQuantityType, HasUserIdType};

#[cgp_component {
    provider: UserBalanceQuerier,
}]
#[async_trait]
pub trait CanQueryCurrentUserBalance:
    HasUserIdType + HasCurrencyType + HasQuantityType + HasAsyncErrorType
{
    async fn query_current_user_balance(
        &self,
        currency: &Self::Currency,
    ) -> Result<Self::Quantity, Self::Error>;
}

#[cgp_component {
    provider: MoneyTransferrer,
}]
#[async_trait]
pub trait CanTransferMoney:
    HasUserIdType + HasCurrencyType + HasQuantityType + HasAsyncErrorType
{
    async fn transfer_money(
        &self,
        recipient: &Self::UserId,
        currency: &Self::Currency,
        quantity: &Self::Quantity,
    ) -> Result<(), Self::Error>;
}
