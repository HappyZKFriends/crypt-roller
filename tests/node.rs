mod test_utils;

use std::collections::HashMap;
use std::collections::HashSet;

use crypt_roller::node::history::TransactionBatch;
use crypt_roller::node::state::AccountState;
use crypt_roller::node::Node;
use crypt_roller::transaction::make_enter;
use crypt_roller::transaction::make_transfer;
use crypt_roller::transaction::Address;

use test_utils::install_fixture;
use test_utils::temporary_test_dir;

#[test]
fn start_empty_and_store() {
    let test_dir = temporary_test_dir("node");
    debug_assert!(!test_dir.path().join("mempool.json").exists());
    debug_assert!(!test_dir.path().join("history.json").exists());

    let node = Node::start(test_dir.path()).unwrap();
    assert!(node.mempool.is_empty());
    assert!(node.history.is_empty());
    assert_eq!(*node.state.batch_count(), 0);
    assert_eq!(node.state.accounts().len(), 0);
    assert!(!test_dir.path().join("mempool.json").exists());
    assert!(!test_dir.path().join("history.json").exists());

    let save_result = node.update_storage(test_dir.path());
    assert!(save_result.is_ok());
    assert!(test_dir.path().join("mempool.json").is_file());
    assert!(test_dir.path().join("history.json").is_file());
}

#[test]
fn start_non_empty_publish_store_and_reload() {
    let test_dir = temporary_test_dir("node");
    install_fixture("history.json", test_dir.path());
    install_fixture("mempool.json", test_dir.path());

    // Load state from disk
    let mut node = Node::start(test_dir.path()).unwrap();

    let expected_initial_mempool = HashSet::from([
        make_transfer(3, 2, 10, 0),
        make_transfer(2, 1, 1, 0),
        make_transfer(1, 2, 3, 1),
        make_enter(4, 100),
        make_enter(4, 50),
    ]);
    let expected_initial_history = [
        vec![make_enter(0, 0), make_enter(1, 10)],
        vec![make_enter(2, 20), make_transfer(1, 0, 5, 0)],
        vec![],
        vec![
            make_transfer(0, 1, 5, 0),
            make_transfer(2, 1, 10, 0),
            make_enter(3, 30),
            make_transfer(2, 1, 5, 1),
        ],
    ];
    let expected_initial_state: HashMap<Address, AccountState> = HashMap::from([
        (0, AccountState::new(0, 1)),
        (1, AccountState::new(25, 1)),
        (2, AccountState::new(5, 2)),
        (3, AccountState::new(30, 0)),
    ]);

    assert_eq!(*node.mempool.transactions(), expected_initial_mempool);
    assert_eq!(*node.state.batch_count(), expected_initial_history.len());
    assert_eq!(*node.state.accounts(), expected_initial_state);
    assert_eq!(node.history.len(), expected_initial_history.len());
    for (actual_batch, expected_batch) in
        node.history.batches().iter().zip(&expected_initial_history)
    {
        assert_eq!(actual_batch.transactions(), expected_batch);
    }

    // Publish new transactions
    let new_transactions = [make_transfer(3, 0, 10, 1), make_transfer(1, 0, 10, 1)];
    for new_transaction in new_transactions.iter() {
        node.mempool.publish_transaction(*new_transaction);
    }

    let expected_mempool_after_publication = expected_initial_mempool
        .union(&HashSet::from(new_transactions))
        .copied()
        .collect();

    assert_eq!(
        *node.mempool.transactions(),
        expected_mempool_after_publication
    );
    assert_eq!(*node.state.batch_count(), expected_initial_history.len());
    assert_eq!(*node.state.accounts(), expected_initial_state);
    assert_eq!(node.history.len(), expected_initial_history.len());
    for (actual_batch, expected_batch) in
        node.history.batches().iter().zip(&expected_initial_history)
    {
        assert_eq!(actual_batch.transactions(), expected_batch);
    }

    // Publish new transaction batches
    let new_batches = [
        vec![make_enter(10, 100), make_enter(11, 110)],
        vec![make_transfer(10, 0, 50, 0), make_transfer(11, 10, 50, 0)],
    ];
    for new_batch in new_batches.iter() {
        node.history
            .publish_batch(TransactionBatch::new(new_batch.clone()));
    }

    let expected_history_after_publication =
        vec![expected_initial_history.as_slice(), new_batches.as_slice()].concat();

    assert_eq!(
        *node.mempool.transactions(),
        expected_mempool_after_publication
    );
    assert_eq!(*node.state.batch_count(), expected_initial_history.len());
    assert_eq!(*node.state.accounts(), expected_initial_state);
    assert_eq!(node.history.len(), expected_history_after_publication.len());
    for (actual_batch, expected_batch) in node
        .history
        .batches()
        .iter()
        .zip(&expected_history_after_publication)
    {
        assert_eq!(actual_batch.transactions(), expected_batch);
    }

    // Save state to disk
    let save_result = node.update_storage(test_dir.path());
    assert!(save_result.is_ok());

    // Load state from disk into a fresh node
    let new_node = Node::start(test_dir.path()).unwrap();

    let expected_state_after_publication: HashMap<Address, AccountState> = HashMap::from([
        (0, AccountState::new(50, 1)),
        (1, AccountState::new(25, 1)),
        (2, AccountState::new(5, 2)),
        (3, AccountState::new(30, 0)),
        (10, AccountState::new(100, 1)),
        (11, AccountState::new(60, 1)),
    ]);

    assert_eq!(
        *new_node.mempool.transactions(),
        expected_mempool_after_publication
    );
    assert_eq!(
        *new_node.state.batch_count(),
        expected_history_after_publication.len()
    );
    assert_eq!(*new_node.state.accounts(), expected_state_after_publication);
    assert_eq!(
        new_node.history.len(),
        expected_history_after_publication.len()
    );
    for (actual_batch, expected_batch) in new_node
        .history
        .batches()
        .iter()
        .zip(&expected_history_after_publication)
    {
        assert_eq!(actual_batch.transactions(), expected_batch);
    }
}
