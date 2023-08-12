use poseidon_rs::{Fr, Poseidon};

use crate::types::types::{ToVecFr, ToU32};

pub trait Hasher {
    /// This function take a slice of bytes and returns the hash of it.
    fn hash(data: &u32) -> u32;
}

#[derive(Default, Clone)]
pub struct PoseidonHasher(Vec<Fr>);

impl Hasher for PoseidonHasher {
    fn hash(data: &u32) -> u32 {
        Poseidon::new().hash(data.to_vec_fr()).unwrap().to_u32()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]

    fn hasher_with_valid_data() {
        let data = 12;
        let data_hashed = PoseidonHasher::hash(&data).to_string();
        //let expected_hash = "0x027ad43cf6415556989fa626bbea0ad4856e5702e493bd6e2e28af8741fce31d"
        //assert_eq!(data_hashed, expected_hash);
    }
}
