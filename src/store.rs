use crate::{Digit, Ternary};
use alloc::string::ToString;
use alloc::vec::Vec;

/// A struct to store 5 ternary digits (~7.8 bits) value into one byte.
///
/// `TritsChunks` helps store ternary numbers into a compact memory structure.
///
/// From `0` to `± 121`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct TritsChunk(i8);

impl TritsChunk {
    pub fn from_dec(from: i8) -> Self {
        Self(from)
    }

    pub fn to_dec(&self) -> i8 {
        self.0
    }

    pub fn to_ternary(&self) -> Ternary {
        Ternary::from_dec(self.0 as i64)
    }

    pub fn to_fixed_ternary(&self) -> Ternary {
        Ternary::from_dec(self.0 as i64).with_length(5)
    }

    pub fn to_digits(&self) -> Vec<Digit> {
        self.to_fixed_ternary().to_digit_slice().to_vec()
    }

    pub fn from_ternary(ternary: Ternary) -> Self {
        if ternary.log() > 5 {
            panic!(
                "TritsChunk::from_ternary(): Ternary is too long: {}",
                ternary.to_string()
            );
        }
        Self(ternary.to_dec() as i8)
    }
}

/// Offers a compact structure to store a ternary number.
///
/// - A [Ternary] is 1 byte long per [Digit]. An 8 (16, 32, 64) digits ternary number is 8 (16, 32, 64) bytes long.
/// - A [DataTernary] is stored into [TritsChunk]. An 8 (16, 32, 64) digits ternary number with this structure is 2 (4, 7, 13) bytes long (1 byte for 5 digits).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DataTernary {
    chunks: Vec<TritsChunk>,
}

impl DataTernary {
    pub fn from_ternary(ternary: Ternary) -> Self {
        let len = ternary.log();
        let diff = 5 - (len % 5);
        let ternary = ternary.with_length(len + diff);
        let mut chunks = Vec::new();
        for i in 0..(ternary.log() / 5) {
            let digits = ternary.to_digit_slice()[i * 5..(i + 1) * 5].to_vec();
            chunks.push(TritsChunk::from_ternary(Ternary::new(digits)));
        }
        Self { chunks }
    }

    pub fn to_ternary(&self) -> Ternary {
        let mut digits = Vec::new();
        for chunk in &self.chunks {
            digits.extend(chunk.to_ternary().to_digit_slice());
        }
        Ternary::new(digits).trim()
    }

    pub fn to_fixed_ternary(&self) -> Ternary {
        let mut digits = Vec::new();
        for chunk in &self.chunks {
            digits.extend(chunk.to_digits());
        }
        Ternary::new(digits).trim()
    }

    pub fn to_digits(&self) -> Vec<Digit> {
        self.to_ternary().trim().to_digit_slice().to_vec()
    }

    pub fn from_dec(from: i64) -> Self {
        Self::from_ternary(Ternary::from_dec(from))
    }

    pub fn to_dec(&self) -> i64 {
        self.to_ternary().to_dec()
    }
}
