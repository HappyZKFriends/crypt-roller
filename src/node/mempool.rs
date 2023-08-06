#[cfg(test)]
mod tests;

use std::collections::HashSet;

use crate::transaction::Transaction;

#[derive(Debug)]
pub struct Mempool {
    transactions: HashSet<Transaction>,
}

impl Mempool {
    pub fn new() -> Self {
        Self {
            transactions: HashSet::new(),
        }
    }

    pub fn transactions(&self) -> &HashSet<Transaction> {
        &self.transactions
    }

    pub fn len(&self) -> usize {
        self.transactions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.transactions.is_empty()
    }

    pub fn publish_transaction(&mut self, transaction: Transaction) {
        if self.transactions.contains(&transaction) {
            println!("Transaction already in the mempool: {:#?}.", transaction);
            return;
        }

        println!(
            "Adding a new transaction to the mempool: {:#?}.",
            transaction
        );
        self.transactions.insert(transaction);
    }

    pub fn drop_transaction(&mut self, transaction: &Transaction) {
        self.transactions.remove(transaction);
    }
}

impl Default for Mempool {
    fn default() -> Self {
        Self::new()
    }
}
