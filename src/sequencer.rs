use crate::node::history::TransactionBatch;
use crate::node::Node;

pub fn build_transaction_batch(node: &Node) -> TransactionBatch {
    // TODO: Invalid transactions should not be published.
    // TODO: There may be conflicting transactions. We can't just always take them all.
    // TODO: Batches should have a limited size.
    TransactionBatch::new(node.mempool.transactions().iter().cloned().collect())
}
