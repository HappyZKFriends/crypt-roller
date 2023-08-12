/// @src main reference source
use crate::types::types::{U32Utils};

pub enum MergeValue {
    Value(u32),
    MergeWithZero {
        base_node: u32,
        zero_bits: u32,
        zero_count: u8
    }
}

impl MergeValue {
    pub fn from_u32(v: u32) -> Self {
        MergeValue::Value(v)
    }

    pub fn zero() -> Self {
        MergeValue::Value(u32::ZERO)
    }

    pub fn is_zero(&self) -> bool {
        match self {
            MergeValue::Value(v) => v.is_zero(),
            MergeValue::MergeWithZero { .. } => false
        }
    }
}

