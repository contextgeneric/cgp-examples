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

#[cgp_provider(UserHashedPasswordQuerierComponent)]
impl<App> UserHashedPasswordQuerier<App> for UseMockedApp
where
    App: HasMockedPasswords + CanRaiseAsyncError<String>,
    App::UserId: Ord,
    App::HashedPassword: Clone,
{
    async fn query_user_hashed_password(
        app: &App,
        user_id: &App::UserId,
    ) -> Result<Option<App::HashedPassword>, App::Error> {
        let hashed_password = app.user_passwords().get(user_id).cloned();

        Ok(hashed_password)
    }
}

#[cgp_provider(PasswordCheckerComponent)]
impl<App> PasswordChecker<App> for UseMockedApp
where
    App: HasPasswordType + HasHashedPasswordType<HashedPassword = App::Password>,
    App::Password: Eq,
{
    fn check_password(password: &App::Password, hashed_password: &App::HashedPassword) -> bool {
        password == hashed_password
    }
}

#[cgp_provider(UserBalanceQuerierComponent)]
impl<App> UserBalanceQuerier<App> for UseMockedApp
where
    App: HasMockedUserBalances + CanRaiseAsyncError<String>,
    App::UserId: Ord + Clone,
    App::Currency: Ord + Clone,
    App::Quantity: Clone,
{
    async fn query_user_balance(
        app: &App,
        user: &App::UserId,
        currency: &App::Currency,
    ) -> Result<App::Quantity, App::Error> {
        let user_balances = app.user_balances().lock().await;

        let user_balance = user_balances
            .get(&(user.clone(), currency.clone()))
            .ok_or_else(|| {
                App::raise_error(format!("user not found in mocked database: {user}"))
            })?;

        Ok(user_balance.clone())
    }
}

#[cgp_provider(MoneyTransferrerComponent)]
impl<App> MoneyTransferrer<App> for UseMockedApp
where
    App: HasMockedUserBalances + CanRaiseAsyncError<String>,
    App::Quantity: CheckedAdd + CheckedSub,
    App::UserId: Ord + Clone,
    App::Currency: Ord + Clone,
{
    async fn transfer_money(
        app: &App,
        sender: &App::UserId,
        recipient: &App::UserId,
        currency: &App::Currency,
        quantity: &App::Quantity,
    ) -> Result<(), App::Error> {
        let mut user_balances = app.user_balances().lock().await;

        let sender_key = (sender.clone(), currency.clone());
        let recipient_key = (recipient.clone(), currency.clone());

        let old_sender_balance = user_balances.get(&sender_key).ok_or_else(|| {
            App::raise_error(format!("sender not found in mocked database: {sender}"))
        })?;

        let old_recipient_balance = user_balances.get(&recipient_key).ok_or_else(|| {
            App::raise_error(format!(
                "recipient not found in mocked database: {recipient}"
            ))
        })?;

        let new_sender_balance = old_sender_balance.checked_sub(quantity)
            .ok_or_else(|| App::raise_error(format!("sender {sender} has insufficient balance {old_sender_balance} to transfer {quantity}")))?;

        let new_recipient_balance = old_recipient_balance
            .checked_add(quantity)
            .ok_or_else(|| App::raise_error(format!("recipient already has too much money!")))?;

        user_balances.insert(sender_key, new_sender_balance);
        user_balances.insert(recipient_key, new_recipient_balance);

        Ok(())
    }
}
