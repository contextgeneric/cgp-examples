use std::collections::BTreeMap;
use std::sync::Arc;

use cgp::prelude::*;
use futures::lock::Mutex;
use num_traits::{CheckedAdd, CheckedSub};

use crate::interfaces::*;
use crate::namespaces::MockNamespace;

/// The in-memory backend provider. One zero-sized marker implements every business and auth
/// capability by reading maps stored on the context, so a deployment swaps the whole backend
/// by wiring these components to a different provider. Each impl below uses `#[default_impl]`
/// to register itself as the `MockNamespace` default for the component it satisfies.
pub struct UseMockedApp;

// Look up a stored password in the in-memory `user_passwords` map. The `#[implicit]` argument
// reads that same-named field off the context; `#[default_impl(... in MockNamespace)]`
// registers this provider as the namespace's default for the password-querier component.
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

// Check a password by plain equality — the mock stores passwords in the clear. The
// `#[use_type]` equality form unifies `HashedPassword` with `Password`, so both are the same
// type and `==` type-checks. A real deployment would swap this for a hashing check.
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

// Read a user's balance from the in-memory `user_balances` map, or raise a 404 if absent.
// The map is pulled in by reference as an `#[implicit]` argument (no clone), and
// `#[uses(...)]` declares the error-raising dependency the not-found branch needs.
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

// Move funds between two users in the in-memory map, checking both accounts exist and that
// the sender has enough balance, raising a 404 or 400 otherwise.
//
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
