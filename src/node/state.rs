use std::collections::HashMap;

use crate::transaction::Address;
use crate::transaction::Amount;
use crate::transaction::Enter;
use crate::transaction::Nonce;
use crate::transaction::Transaction;
use crate::transaction::Transfer;

#[derive(Debug)]
pub enum TransactionExecutionError {
    SourceAccountMissing {
        transfer: Transfer,
    },
    TargetAccountMissing {
        transfer: Transfer,
    },
    WrongNonce {
        transfer: Transfer,
        account: AccountState,
    },
    InsufficientBalance {
        account: AccountState,
        amount: Amount,
    },
    BalanceOverflow {
        account: AccountState,
        amount: Amount,
    },
    NonceOverflow,
}

#[derive(Clone, Debug)]
pub struct AccountState {
    balance: Amount,
    next_nonce: Nonce,
}

impl AccountState {
    pub fn new(balance: Amount) -> Self {
        AccountState {
            balance,
            next_nonce: 0,
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
                account: self.clone(),
                amount,
            });
        }

        if amount > Amount::MAX - self.balance {
            return Err(TransactionExecutionError::BalanceOverflow {
                account: to_account.clone(),
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

#[derive(Debug)]
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
                    .or_insert(AccountState::new(0))
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
                    transfer: transfer.clone(),
                },
            );
        };

        let Some(_to_account) = self.accounts.get(&transfer.to) else {
            return Err(
                TransactionExecutionError::TargetAccountMissing {
                    transfer: transfer.clone(),
                },
            );
        };

        if transfer.nonce != from_account.next_nonce {
            return Err(TransactionExecutionError::WrongNonce {
                transfer: transfer.clone(),
                account: from_account.clone(),
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
