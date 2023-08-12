use ff::PrimeField;
use poseidon_rs::{Fr, Poseidon};
use ethers_core::types::U256;

pub trait Hasher {
    type Hash: ToString;

    fn to_vec_fr(data: &str) -> Vec<Fr>;

    /// This function take a slice of bytes and returns the hash of it.
    fn hash(data: Vec<Fr>) -> Fr;
}

#[derive(Default, Clone)]
pub struct PoseidonHasher(Vec<Fr>);

impl Hasher for PoseidonHasher {
    type Hash = Fr;

    fn to_vec_fr(data: &str) -> Vec<Fr> {
        let data_fr = Fr::from_str(data).unwrap();
        let data_vec_fr = vec![data_fr];
        data_vec_fr
    }

    /// @read https://blog.burntsushi.net/unwrap/
    fn hash(data: Vec<Fr>) -> Self::Hash {
        Poseidon::new().hash(data).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]

    fn hasher_with_valid_data() {
        let data: &str = "1234";
        let vec_fr_data = PoseidonHasher::to_vec_fr(data);
        let data_hashed = PoseidonHasher::hash(vec_fr_data).to_string();
        //let expected_hash = "0x027ad43cf6415556989fa626bbea0ad4856e5702e493bd6e2e28af8741fce31d"
        //assert_eq!(data_hashed, expected_hash);
    }
}
