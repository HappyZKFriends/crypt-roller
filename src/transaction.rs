pub type Address = u32;
pub type Amount = u16;
pub type Nonce = u32;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Enter {
    pub account: Address,
    pub amount: Amount,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Transfer {
    pub from: Address,
    pub to: Address,
    pub amount: Amount,
    pub nonce: Nonce,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Transaction {
    Enter(Enter),
    Transfer(Transfer),
}

pub fn make_transfer(from: Address, to: Address, amount: Amount, nonce: Nonce) -> Transaction {
    Transaction::Transfer(Transfer {
        from,
        to,
        amount,
        nonce,
    })
}

pub fn make_enter(account: Address, amount: Amount) -> Transaction {
    Transaction::Enter(Enter { account, amount })
}
