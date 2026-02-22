use std::collections::BTreeMap;
use std::sync::Arc;

use cgp::prelude::*;
use futures::lock::Mutex;
use num_traits::{CheckedAdd, CheckedSub};

use crate::interfaces::*;

pub struct UseMockedApp;

#[cgp_auto_getter]
pub trait HasMockedUserBalances: HasUserIdType + HasCurrencyType + HasQuantityType {
    fn user_balances(
        &self,
    ) -> &Arc<Mutex<BTreeMap<(Self::UserId, Self::Currency), Self::Quantity>>>;
}

#[cgp_auto_getter]
pub trait HasMockedPasswords: HasUserIdType + HasHashedPasswordType {
    fn user_passwords(&self) -> &BTreeMap<Self::UserId, Self::HashedPassword>;
}

#[cgp_impl(UseMockedApp)]
impl UserHashedPasswordQuerier
where
    Self: HasMockedPasswords + HasErrorType,
    Self::UserId: Ord,
    Self::HashedPassword: Clone,
{
    async fn query_user_hashed_password(
        &self,
        user_id: &Self::UserId,
    ) -> Result<Option<Self::HashedPassword>, Self::Error> {
        let hashed_password = self.user_passwords().get(user_id).cloned();

        Ok(hashed_password)
    }
}

#[cgp_impl(UseMockedApp)]
impl PasswordChecker
where
    Self: HasPasswordType + HasHashedPasswordType<HashedPassword = Self::Password>,
    Self::Password: Eq,
{
    fn check_password(password: &Self::Password, hashed_password: &Self::HashedPassword) -> bool {
        password == hashed_password
    }
}

#[cgp_impl(UseMockedApp)]
impl UserBalanceQuerier
where
    Self: HasMockedUserBalances + CanRaiseHttpError<ErrNotFound, String>,
    Self::UserId: Ord + Clone,
    Self::Currency: Ord + Clone,
    Self::Quantity: Clone,
{
    async fn query_user_balance(
        &self,
        user: &Self::UserId,
        currency: &Self::Currency,
    ) -> Result<Self::Quantity, Self::Error> {
        let user_balances = self.user_balances().lock().await;

        let user_balance = user_balances
            .get(&(user.clone(), currency.clone()))
            .ok_or_else(|| {
                Self::raise_http_error(
                    ErrNotFound,
                    format!("user not found in mocked database: {user}"),
                )
            })?;

        Ok(user_balance.clone())
    }
}

#[cgp_impl(UseMockedApp)]
impl MoneyTransferrer
where
    Self: HasMockedUserBalances
        + CanRaiseHttpError<ErrNotFound, String>
        + CanRaiseHttpError<ErrBadRequest, String>,
    Self::Quantity: CheckedAdd + CheckedSub,
    Self::UserId: Ord + Clone,
    Self::Currency: Ord + Clone,
{
    async fn transfer_money(
        &self,
        sender: &Self::UserId,
        recipient: &Self::UserId,
        currency: &Self::Currency,
        quantity: &Self::Quantity,
    ) -> Result<(), Self::Error> {
        let mut user_balances = self.user_balances().lock().await;

        let sender_key = (sender.clone(), currency.clone());
        let recipient_key = (recipient.clone(), currency.clone());

        let old_sender_balance = user_balances.get(&sender_key).ok_or_else(|| {
            Self::raise_http_error(
                ErrNotFound,
                format!("sender not found in mocked database: {sender}"),
            )
        })?;

        let old_recipient_balance = user_balances.get(&recipient_key).ok_or_else(|| {
            Self::raise_http_error(
                ErrNotFound,
                format!("recipient not found in mocked database: {recipient}"),
            )
        })?;

        let new_sender_balance = old_sender_balance.checked_sub(quantity)
            .ok_or_else(|| Self::raise_http_error(ErrBadRequest, format!("sender {sender} has insufficient balance {old_sender_balance} to transfer {quantity}")))?;

        let new_recipient_balance =
            old_recipient_balance.checked_add(quantity).ok_or_else(|| {
                Self::raise_http_error(
                    ErrBadRequest,
                    "recipient already has too much money!".to_string(),
                )
            })?;

        user_balances.insert(sender_key, new_sender_balance);
        user_balances.insert(recipient_key, new_recipient_balance);

        Ok(())
    }
}
