use std::collections::BTreeMap;
use std::sync::Arc;

use cgp::prelude::*;
use futures::lock::Mutex;
use num_traits::{CheckedAdd, CheckedSub};

use crate::interfaces::*;
use crate::namespaces::MockNamespace;

pub struct UseMockedApp;

#[cgp_impl(UseMockedApp)]
#[default_impl(@app.auth.UserHashedPasswordQuerierComponent in MockNamespace)]
#[use_type(HasUserIdType.UserId, HasHashedPasswordType.HashedPassword, HasErrorType.Error)]
impl UserHashedPasswordQuerier
where
    UserId: Ord,
    HashedPassword: Clone,
{
    async fn query_user_hashed_password(
        &self,
        user_id: &UserId,
        #[implicit] user_passwords: &BTreeMap<UserId, HashedPassword>,
    ) -> Result<Option<HashedPassword>, Error> {
        let hashed_password = user_passwords.get(user_id).cloned();

        Ok(hashed_password)
    }
}

#[cgp_impl(UseMockedApp)]
#[default_impl(@app.auth.PasswordCheckerComponent in MockNamespace)]
#[use_type(HasPasswordType.Password, HasHashedPasswordType.{HashedPassword = Password})]
impl PasswordChecker
where
    Password: Eq,
{
    fn check_password(password: &Password, hashed_password: &HashedPassword) -> bool {
        password == hashed_password
    }
}

#[cgp_impl(UseMockedApp)]
#[default_impl(@app.finance.UserBalanceQuerierComponent in MockNamespace)]
#[uses(CanRaiseHttpError<ErrNotFound, String>)]
#[use_type(HasUserIdType.UserId, HasCurrencyType.Currency, HasQuantityType.Quantity, HasErrorType.Error)]
impl UserBalanceQuerier
where
    UserId: Ord + Clone,
    Currency: Ord + Clone,
    Quantity: Clone,
{
    async fn query_user_balance(
        &self,
        user: &UserId,
        currency: &Currency,
        #[implicit] user_balances: &Arc<Mutex<BTreeMap<(UserId, Currency), Quantity>>>,
    ) -> Result<Quantity, Error> {
        let balances = user_balances.lock().await;

        let user_balance = balances
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

// Note: `MoneyTransferrer` is deliberately *not* registered into `MockNamespace`.
// `MockApp` overrides it directly with `NoTransferToSelf<UseMockedApp>` on the
// `@app.finance.MoneyTransferrerComponent` path, and a context can only wire a
// path the joined namespace does not itself claim — registering it here too would
// make the context entry conflict with the namespace's blanket forwarding impl.
#[cgp_impl(UseMockedApp)]
#[uses(CanRaiseHttpError<ErrNotFound, String>, CanRaiseHttpError<ErrBadRequest, String>)]
#[use_type(HasUserIdType.UserId, HasCurrencyType.Currency, HasQuantityType.Quantity, HasErrorType.Error)]
impl MoneyTransferrer
where
    Quantity: CheckedAdd + CheckedSub,
    UserId: Ord + Clone,
    Currency: Ord + Clone,
{
    async fn transfer_money(
        &self,
        sender: &UserId,
        recipient: &UserId,
        currency: &Currency,
        quantity: &Quantity,
        #[implicit] user_balances: &Arc<Mutex<BTreeMap<(UserId, Currency), Quantity>>>,
    ) -> Result<(), Error> {
        let mut balances = user_balances.lock().await;

        let sender_key = (sender.clone(), currency.clone());
        let recipient_key = (recipient.clone(), currency.clone());

        let old_sender_balance = balances.get(&sender_key).ok_or_else(|| {
            Self::raise_http_error(
                ErrNotFound,
                format!("sender not found in mocked database: {sender}"),
            )
        })?;

        let old_recipient_balance = balances.get(&recipient_key).ok_or_else(|| {
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

        balances.insert(sender_key, new_sender_balance);
        balances.insert(recipient_key, new_recipient_balance);

        Ok(())
    }
}
