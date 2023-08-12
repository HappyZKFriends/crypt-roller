use ff::PrimeField;
use poseidon_rs::{Fr};

use crate::prover::sparse_merkle_tree::{BranchKey, BranchNode};
use crate::prover::errors::MerkleTreeError;

const ZERO_U32: u32 = 0;

/// Fr traits
pub trait ToFr {
    fn to_fr(&self) -> Fr;
}

/// Convert a u32 into a field element.
impl ToFr for u32 {
    fn to_fr(&self) -> Fr {
        Fr::from_str(&self.to_string()).unwrap()
    }
}

pub trait ToVecFr {
    fn to_vec_fr(&self) -> Vec<Fr>;
}

/// Convert a u32 into a vector of field element.
impl ToVecFr for u32 {
    fn to_vec_fr(&self) -> Vec<Fr> {
        vec![Fr::from_str(&self.to_string()).unwrap()]
    }
}

/// u32 traits
pub trait ToU32 {
    fn to_u32(&self) -> u32;
}

impl ToU32 for Fr {
    fn to_u32(&self) -> u32 {
        u32::from_str_radix(&ff::to_hex(self), 16).unwrap()
    }
}

pub trait U32Utils {
    const ZERO: u32;
    fn is_zero(&self) -> bool;
}

impl U32Utils for u32 {
    const ZERO: u32 = ZERO_U32;

    fn is_zero(&self) -> bool {
        self == &ZERO_U32
    }
}


/// Store traits
pub trait StoreReadOps<V> {
    fn get_branch(&self, branch_key: &BranchKey) -> Result<Option<BranchNode>, MerkleTreeError>;
    fn get_leaf(&self, leaf_key: &u32) -> Result<Option<V>, MerkleTreeError>;
}
