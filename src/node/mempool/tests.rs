use crate::transaction::make_enter;
use crate::transaction::make_transfer;

use super::Mempool;

#[test]
fn new_mempool_should_be_empty() {
    assert!(Mempool::new().is_empty());
}

#[test]
fn publish_transaction_should_add_transaction_to_mempool() {
    let mut mempool = Mempool::new();

    mempool.publish_transaction(make_enter(0, 10));
    assert_eq!(mempool.len(), 1);
    mempool.publish_transaction(make_enter(1, 0));
    assert_eq!(mempool.len(), 2);
    mempool.publish_transaction(make_transfer(0, 1, 10, 0));
    assert_eq!(mempool.len(), 3);

    assert!(mempool.transactions.contains(&make_enter(0, 10)));
    assert!(mempool.transactions.contains(&make_enter(1, 0)));
    assert!(mempool.transactions.contains(&make_transfer(0, 1, 10, 0)));
}

#[test]
fn publish_transaction_should_accept_invalid_transactions() {
    let mut mempool = Mempool::new();

    mempool.publish_transaction(make_transfer(0, 0, 100, 100));
}

#[test]
fn drop_transaction_should_remove_transaction_from_mempool_if_exists() {
    let mut mempool = Mempool::new();
    mempool.transactions.insert(make_enter(0, 10));
    mempool.transactions.insert(make_enter(1, 0));

    mempool.drop_transaction(&make_enter(0, 10));
    assert_eq!(mempool.len(), 1);
    assert!(!mempool.transactions.contains(&make_enter(0, 10)));
    assert!(mempool.transactions.contains(&make_enter(1, 0)));
}
