#[cfg(test)]
mod tests;

use std::path::Path;

use serde::Deserialize;
use serde::Serialize;

use crate::transaction::Transaction;
use crate::utils::storage::load_json;
use crate::utils::storage::save_json;
use crate::utils::storage::StorageError;

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct TransactionBatch {
    transactions: Vec<Transaction>,
}

impl TransactionBatch {
    pub fn new(transactions: Vec<Transaction>) -> Self {
        Self { transactions }
    }

    pub fn transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }

    pub fn len(&self) -> usize {
        self.transactions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.transactions.is_empty()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct History {
    batches: Vec<TransactionBatch>,
}

impl History {
    pub fn new() -> Self {
        Self { batches: vec![] }
    }

    pub fn len(&self) -> usize {
        self.batches.len()
    }

    pub fn is_empty(&self) -> bool {
        self.batches.is_empty()
    }

    pub fn batches(&self) -> &Vec<TransactionBatch> {
        &self.batches
    }

    pub fn load(storage_path: &Path) -> Result<Self, StorageError> {
        load_json(storage_path, Self::new())
    }

    pub fn save(&self, storage_path: &Path) -> Result<(), StorageError> {
        save_json(storage_path, self)
    }

    pub fn publish_batch(&mut self, batch: TransactionBatch) {
        println!("Publishing a new batch of {} transactions.", batch.len());

        // TODO: Validate correctness of transactions
        self.batches.push(batch);
    }
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}
