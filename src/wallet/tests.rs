use crate::node::Node;
use crate::transaction::make_enter;

use super::Wallet;

#[test]
fn build_enter_transaction_should_create_transaction() {
    let node = Node::new();
    let result = Wallet::build_enter_transaction(10, &node);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), (Wallet { account: 0 }, make_enter(0, 10)));
}
