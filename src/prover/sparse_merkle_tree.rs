use poseidon_rs::{Fr, Poseidon};

use core::marker::PhantomData;
use core::cmp::Ordering;

/// Branch key
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct BranchKey {
    pub height: u8,
    pub node_key: Fr,
}

impl BranchKey {
    pub fn new(height: u8, node_key: Fr) -> BranchKey {
        BranchKey { height, node_key }
    }
}

impl Ord for BranchKey {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.height.cmp(&other.height) {
            Ordering::Equal => self.node_key.cmp(&other.node_key),
            ordering => ordering
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
    pub right: MergeValue 
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
