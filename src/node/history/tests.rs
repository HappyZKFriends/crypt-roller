use crate::transaction::make_enter;
use crate::transaction::make_transfer;

use super::History;
use super::TransactionBatch;

#[test]
fn new_transaction_batch_should_be_empty() {
    assert!(TransactionBatch::new(vec![]).is_empty());
}

#[test]
fn new_history_batch_should_be_empty() {
    assert!(History::new().batches().is_empty());
}

#[test]
fn publish_batch_should_append_empty_batch_to_history() {
    let mut history = History::new();

    history.publish_batch(TransactionBatch::new(vec![]));
    assert_eq!(history.batches().len(), 1);
    assert!(history.batches()[0].is_empty());
}

#[test]
fn publish_batch_should_append_non_empty_batches_to_history() {
    let mut history = History::new();

    let batch = TransactionBatch::new(vec![
        make_enter(0, 10),
        make_enter(1, 0),
        make_transfer(0, 1, 10, 0),
    ]);
    history.publish_batch(batch.clone());
    assert_eq!(history.batches().len(), 1);
    assert_eq!(history.batches()[0], batch);
}
