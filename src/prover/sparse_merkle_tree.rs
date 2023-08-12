use poseidon_rs::{Fr, Poseidon};

use core::cmp::Ordering;
use core::marker::PhantomData;

use crate::prover::merge::MergeValue;
use crate::prover::poseidon_hasher::Hasher;
use crate::types::types::{StoreReadOps, U32Utils};

/// Branch key
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct BranchKey {
    pub height: u8,
    pub node_key: u32,
}

impl BranchKey {
    pub fn new(height: u8, node_key: u32) -> BranchKey {
        BranchKey { height, node_key }
    }
}

impl Ord for BranchKey {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.height.cmp(&other.height) {
            Ordering::Equal => self.node_key.cmp(&other.node_key),
            ordering => ordering,
        }
    }
}

impl PartialOrd for BranchKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Branch Node
pub struct BranchNode {
    pub left: MergeValue,
    pub right: MergeValue,
}

impl BranchNode {
    /// Creating a new empty branches
    pub fn new_empty() -> BranchNode {
        BranchNode {
            left: MergeValue::zero(),
            right: MergeValue::zero(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.left.is_zero() && self.right.is_zero()
    }
}

/// Sparse merkle tree
pub struct SparseMerkleTree<H, V, S> {
    store: S,
    root: Fr,
    phantom: PhantomData<(H, V)>,
}

impl<H, V, S> SparseMerkleTree<H, V, S> {
    pub fn new(root: Fr, store: S) -> SparseMerkleTree<H, V, S> {
        SparseMerkleTree {
            store,
            root,
            phantom: PhantomData,
        }
    }

    pub fn root(&self) -> &Fr {
        &self.root
    }

    pub fn is_empty(&self) -> bool {
        self.root == Fr::default()
    }

    pub fn take_store(self) -> S {
        self.store
    }

    pub fn store(&self) -> &S {
        &self.store
    }

    /// Get mutable backend store
    pub fn store_mut(&mut self) -> &mut S {
        &mut self.store
    }
}

impl<H: Hasher + Default, V, S: StoreReadOps<V>> SparseMerkleTree<H, V, S> {
    /// Build a merkle tree from store, the root will be calculated automatically
    pub fn new_with_store(store: S) -> Result<SparseMerkleTree<H, V, S>> {
        let root_branch_key = BranchKey::new(u8::MAX, u32::ZERO);
        store
            .get_branch(&root_branch_key)
            .map(|branch_node| { 
                branch_node
                    .map(|n| {
                           
                    })
        })
    }
}
