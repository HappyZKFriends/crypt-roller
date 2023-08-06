use std::collections::HashMap;

use thiserror::Error;

use crate::transaction::Address;
use crate::transaction::Amount;
use crate::transaction::Enter;
use crate::transaction::Nonce;
use crate::transaction::Transaction;
use crate::transaction::Transfer;

#[derive(Error, Debug)]
pub enum TransactionExecutionError {
    #[error("Source account {} does not exist.", .transfer.from)]
    SourceAccountMissing { transfer: Transfer },

    #[error("Target account {} does not exist.", .transfer.from)]
    TargetAccountMissing { transfer: Transfer },

    #[error("Wrong nonce. Expected {}, got {}.", .account.next_nonce, .transfer.nonce)]
    WrongNonce {
        transfer: Transfer,
        account: AccountState,
    },

    #[error(
        "Insufficient balance. Attempted to transfer {} out of an account with balance {}.",
        .amount,
        .account.balance,
    )]
    InsufficientBalance {
        account: AccountState,
        amount: Amount,
    },

    #[error("Balance overflow. Transferring {} to an account with balance {} would bring it over the maximum of {}.",
        .amount,
        .account.balance,
        Amount::MAX,
    )]
    BalanceOverflow {
        account: AccountState,
        amount: Amount,
    },

    #[error("Nonce overflow.")]
    NonceOverflow,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct AccountState {
    balance: Amount,
    next_nonce: Nonce,
}

impl AccountState {
    pub fn new(balance: Amount, next_nonce: Nonce) -> Self {
        AccountState {
            balance,
            next_nonce,
        }
    }

    pub fn balance(&self) -> &Amount {
        &self.balance
    }

    pub fn next_nonce(&self) -> &Nonce {
        &self.next_nonce
    }

    pub fn transfer(
        &mut self,
        to_account: &mut Self,
        amount: Amount,
    ) -> Result<(), TransactionExecutionError> {
        if amount > self.balance {
            return Err(TransactionExecutionError::InsufficientBalance {
                account: *self,
                amount,
            });
        }

        if amount > Amount::MAX - self.balance {
            return Err(TransactionExecutionError::BalanceOverflow {
                account: *to_account,
                amount,
            });
        }

        if self.next_nonce == Nonce::MAX {
            return Err(TransactionExecutionError::NonceOverflow);
        }

        self.balance -= amount;
        self.next_nonce += 1;
        to_account.balance += amount;
        Ok(())
    }
}

impl Default for AccountState {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct RollupState {
    accounts: HashMap<Address, AccountState>,
    batch_count: usize,
}

impl RollupState {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            batch_count: 0,
        }
    }

    pub fn accounts(&self) -> &HashMap<Address, AccountState> {
        &self.accounts
    }

    pub fn batch_count(&self) -> &usize {
        &self.batch_count
    }

    pub fn apply_transaction(
        &mut self,
        transaction: &Transaction,
    ) -> Result<(), TransactionExecutionError> {
        match transaction {
            Transaction::Enter(Enter { account, amount }) => {
                self.accounts
                    .entry(*account)
                    .or_insert(AccountState::default())
                    .balance += amount;
            }
            Transaction::Transfer(
                transfer @ Transfer {
                    from: from_address,
                    to: to_address,
                    amount,
                    nonce: _,
                },
            ) => {
                self.validate_addresses(transfer)?;

                let mut from_account = self.accounts.remove(from_address).unwrap();
                let mut to_account = self.accounts.remove(to_address).unwrap();
                from_account.transfer(&mut to_account, *amount)?;
                self.accounts.insert(*from_address, from_account);
                self.accounts.insert(*to_address, to_account);
            }
        }

        Ok(())
    }

    pub fn apply_batch(
        &mut self,
        transactions: &[Transaction],
    ) -> Result<(), TransactionExecutionError> {
        for transaction in transactions.iter() {
            self.apply_transaction(transaction)?;
        }
        self.batch_count += 1;
        Ok(())
    }

    fn validate_addresses(&self, transfer: &Transfer) -> Result<(), TransactionExecutionError> {
        let Some(from_account) = self.accounts.get(&transfer.from) else {
            return Err(
                TransactionExecutionError::SourceAccountMissing {
                    transfer: *transfer,
                },
            );
        };

        let Some(_to_account) = self.accounts.get(&transfer.to) else {
            return Err(
                TransactionExecutionError::TargetAccountMissing {
                    transfer: *transfer,
                },
            );
        };

        if transfer.nonce != from_account.next_nonce {
            return Err(TransactionExecutionError::WrongNonce {
                transfer: *transfer,
                account: *from_account,
            });
        }

        Ok(())
    }
}

impl Default for RollupState {
    fn default() -> Self {
        Self::new()
    }
}
