pub mod history;
pub mod mempool;
pub mod state;

use std::path::Path;

use thiserror::Error;

use crate::utils::storage::StorageError;

use history::History;
use mempool::Mempool;
use state::RollupState;
use state::TransactionExecutionError;

#[derive(Error, Debug)]
pub enum NodeError {
    #[error("Error while executing transaction | {0}")]
    Transaction(#[from] TransactionExecutionError),

    #[error("Storage error | {0}")]
    Storage(#[from] StorageError),
}

#[derive(Debug)]
pub struct Node {
    pub mempool: Mempool,
    pub history: History,
    pub state: RollupState,
}

impl Node {
    pub fn new() -> Self {
        Self {
            mempool: Mempool::new(),
            history: History::new(),
            state: RollupState::new(),
        }
    }

    pub fn start(storage_dir: &Path) -> Result<Self, NodeError> {
        let mut node = Self {
            mempool: Mempool::load(&storage_dir.join("mempool.json"))?,
            history: History::load(&storage_dir.join("history.json"))?,
            state: RollupState::new(),
        };

        node.apply_history()?;
        Ok(node)
    }

    pub fn update_storage(&self, storage_dir: &Path) -> Result<(), NodeError> {
        self.history.save(&storage_dir.join("history.json"))?;
        self.mempool.save(&storage_dir.join("mempool.json"))?;
        Ok(())
    }

    fn apply_history(&mut self) -> Result<(), NodeError> {
        debug_assert!(*self.state.batch_count() <= self.history.len());

        let new_batches = self.history.len() - self.state.batch_count();
        if new_batches == 0 {
            println!("Node state already up to date with history.");
            return Ok(());
        }

        println!(
            "Applying {} transaction batches to rollup state:",
            new_batches
        );
        for batch in self.history.batches()[*self.state.batch_count()..self.history.len()].iter() {
            println!("    -> Applying {} transactions.", batch.len());
            self.state.apply_batch(batch.transactions())?;
            for transaction in batch.transactions().iter() {
                self.mempool.drop_transaction(transaction);
            }
        }

        Ok(())
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::new()
    }
}
