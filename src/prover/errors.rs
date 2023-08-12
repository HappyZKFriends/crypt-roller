use core::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MerkleTreeError {
    MissingBranch(u8, u32),
    MissingLeaf(u32),
}

impl Display for MerkleTreeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            MerkleTreeError::MissingBranch(height, key) => {
                write!(
                    f,
                    "MerkleTreeError: Corrupted store, missing branch height: {}, key: {}",
                    height, key
                )?;
            }
            MerkleTreeError::MissingLeaf(key) => write!(
                f,
                "MerkleTreeError: Corrupted store, missing leaf {:?}",
                key
            )?,
        }
        Ok(())
    }
}
