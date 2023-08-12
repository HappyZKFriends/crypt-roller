/// This code is for learning purposes from this @src:https://github.com/antouhou/rs-merkle/blob/master/src/partial_tree.rs
use crate::poseidon_hasher::Hasher;


type PartialTreeLayer<H> = Vec<(usize, H)>;

pub struct PartialTree<T: Hasher> {
    layers: Vec<PartialTreeLayer<T>>
}

impl<T: Hasher> Default for PartialTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Hasher> PartialTree<T> {
    pub fn new() -> Self {
        Self {
            layers: Vec::new()
        }
    }

    pub fn from_leaves(leaves: &[T::Hash]) -> Result<Self, Error> {
        let leaf_tuples: Vec<(usize, T::Hash)> = leaves.iter().cloned().enumerate().collect();

        
    }

    fn build_tree(
        mut partial_layers: Vec<Vec<(usize, T::Hash)>>,
        full_tree_depth: usize,
    ) -> Result<Vec<PartialTreeLayer<T::Hash>>, Error> {

    }
}
