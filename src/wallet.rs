#[cfg(test)]
mod tests;

use crate::node::Node;
use crate::transaction::Address;
use crate::transaction::Amount;
use crate::transaction::Enter;
use crate::transaction::Transaction;
use crate::transaction::Transfer;

#[derive(Debug)]
pub enum WalletError {
    InsufficientBalance,
    InvalidAddress,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Wallet {
    // TODO: Wallet initialization
    pub account: Address,
}

impl Wallet {
    pub fn build_transfer_transaction(
        self,
        to: Address,
        amount: Amount,
        node: &Node,
    ) -> Result<Transaction, WalletError> {
        let Some(account_state) = node.state.accounts().get(&self.account) else {
            return Err(WalletError::InvalidAddress);
        };

        // TODO: More detailed validations. Nonce, balance overflow, etc.
        if amount > *account_state.balance() {
            return Err(WalletError::InsufficientBalance);
        }

        Ok(Transaction::Transfer(Transfer {
            from: self.account,
            to,
            amount,
            nonce: *account_state.next_nonce(),
        }))
    }

    pub fn build_enter_transaction(
        amount: Amount,
        node: &Node,
    ) -> Result<(Wallet, Transaction), WalletError> {
        // TODO: Make this smarter - check mempool for pending transactions or remember more state.
        // Currently all transactions get the same nonce until a new batch is published.
        // Also, all enter transactions in the same batch end up referring to the same account.
        let account = node.state.accounts().len() as Address;
        let wallet = Wallet { account };

        Ok((wallet, Transaction::Enter(Enter { account, amount })))
    }
}
