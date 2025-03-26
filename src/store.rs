use crate::concepts::DigitOperate;
use crate::{Digit, Ternary};
use alloc::string::ToString;
use alloc::vec::Vec;
use core::fmt::Display;

/// A struct to store 5 ternary digits (~7.8 bits) value into one byte.
///
/// `TritsChunks` helps store ternary numbers into a compact memory structure.
///
/// From `0` to `± 121`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct TritsChunk(i8);

impl TritsChunk {
    /// Creates a `TritsChunk` from a given decimal value.
    ///
    /// # Arguments
    ///
    /// * `from` - An `i8` value representing the decimal value to be converted into a `TritsChunk`.
    ///
    /// # Panics
    ///
    /// This function panics if the input value is out of the valid range `-121..=121`.
    ///
    /// # Example
    ///
    /// ```
    /// use balanced_ternary::TritsChunk;
    ///
    /// let chunk = TritsChunk::from_dec(42);
    /// assert_eq!(chunk.to_dec(), 42);
    /// ```
    pub fn from_dec(from: i8) -> Self {
        if !(-121..=121).contains(&from) {
            panic!("TritsChunk::from_dec(): Invalid value: {}", from);
        }
        Self(from)
    }

    /// Converts the `TritsChunk` into its decimal representation.
    ///
    /// # Returns
    ///
    /// An `i8` value representing the decimal form of the `TritsChunk`.
    ///
    /// # Example
    ///
    /// ```
    /// use balanced_ternary::TritsChunk;
    ///
    /// let chunk = TritsChunk::from_dec(42);
    /// assert_eq!(chunk.to_dec(), 42);
    /// ```
    pub fn to_dec(&self) -> i8 {
        self.0
    }

    /// Converts the `TritsChunk` into its ternary representation.
    ///
    /// # Returns
    ///
    /// A `Ternary` type representing the ternary form of the `TritsChunk`.
    ///
    /// # Example
    ///
    /// ```
    /// use balanced_ternary::{TritsChunk, Ternary};
    ///
    /// let chunk = TritsChunk::from_dec(42);
    /// let ternary = chunk.to_ternary();
    /// assert_eq!(ternary.to_dec(), 42);
    /// ```
    pub fn to_ternary(&self) -> Ternary {
        Ternary::from_dec(self.0 as i64)
    }

    /// Converts the `TritsChunk` into its fixed-length ternary representation.
    ///
    /// # Returns
    ///
    /// A `Ternary` type representing the 5-digit fixed-length ternary form of the `TritsChunk`.
    ///
    /// # Example
    ///
    /// ```
    /// use balanced_ternary::{TritsChunk, Ternary};
    ///
    /// let chunk = TritsChunk::from_dec(42);
    /// let fixed_ternary = chunk.to_fixed_ternary();
    /// assert_eq!(fixed_ternary.to_dec(), 42);
    /// assert_eq!(fixed_ternary.to_digit_slice().len(), 5);
    /// ```
    pub fn to_fixed_ternary(&self) -> Ternary {
        Ternary::from_dec(self.0 as i64).with_length(5)
    }

    /// Converts the `TritsChunk` into a vector of its individual ternary digits.
    ///
    /// # Returns
    ///
    /// A `Vec<Digit>` representing the individual ternary digits of the `TritsChunk`.
    ///
    /// The resulting vector will always contain 5 digits since the `TritsChunk` is
    /// represented in a fixed-length ternary form.
    ///
    /// # Example
    ///
    /// ```
    /// use balanced_ternary::{TritsChunk, Digit};
    ///
    /// let chunk = TritsChunk::from_dec(42);
    /// let digits: Vec<Digit> = chunk.to_digits();
    /// assert_eq!(digits.len(), 5);
    /// ```
    pub fn to_digits(&self) -> Vec<Digit> {
        self.to_fixed_ternary().to_digit_slice().to_vec()
    }

    /// Creates a `TritsChunk` from a given `Ternary` value.
    ///
    /// # Arguments
    ///
    /// * `ternary` - A `Ternary` value to be converted into a `TritsChunk`.
    ///
    /// # Panics
    ///
    /// This function panics if the provided `ternary` value has a logarithmic length greater than 5,
    /// indicating that it cannot be represented by a single `TritsChunk`.
    ///
    /// # Example
    ///
    /// ```
    /// use balanced_ternary::{TritsChunk, Ternary};
    ///
    /// let ternary = Ternary::from_dec(42);
    /// let chunk = TritsChunk::from_ternary(ternary);
    /// assert_eq!(chunk.to_dec(), 42);
    /// ```
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
///
/// Use the [Ternary] type to execute operations on numbers and [DataTernary] to store numbers.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DataTernary {
    chunks: Vec<TritsChunk>,
}

impl DataTernary {
    /// Creates a new instance of `DataTernary` from a given `Ternary` value.
    ///
    /// This method ensures that the total number of ternary digits is a multiple of 5
    /// by padding as necessary. It then divides the ternary number into chunks of
    /// 5 digits each, which are stored in the `DataTernary` structure.
    ///
    /// # Arguments
    ///
    /// * `ternary` - A `Ternary` value to be converted into a `DataTernary`.
    ///
    /// # Returns
    ///
    /// A new `DataTernary` instance containing the converted chunks.
    ///
    /// # Example
    ///
    /// ```
    /// use balanced_ternary::{DataTernary, Ternary};
    ///
    /// let ternary = Ternary::from_dec(42);
    /// let data_ternary = DataTernary::from_ternary(ternary);
    /// assert_eq!(data_ternary.to_dec(), 42);
    /// ```
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

    /// Converts a `DataTernary` into its equivalent `Ternary` representation.
    ///
    /// This function iterates over all the `TritsChunk` instances in the `DataTernary`,
    /// extracts their ternary representations, and reconstructs them into the full
    /// `Ternary` value. The resulting `Ternary` value may be trimmed to remove
    /// any leading zeroes in its ternary digit representation.
    ///
    /// # Returns
    ///
    /// A `Ternary` value that represents the combined ternary digits of the
    /// `DataTernary`.
    ///
    /// # Example
    ///
    /// ```
    /// use balanced_ternary::{DataTernary, Ternary};
    ///
    /// let ternary = Ternary::from_dec(42);
    /// let data_ternary = DataTernary::from_ternary(ternary.clone());
    /// assert_eq!(data_ternary.to_ternary(), ternary);
    /// ```
    pub fn to_ternary(&self) -> Ternary {
        let mut digits = Vec::new();
        for chunk in &self.chunks {
            digits.extend(chunk.to_ternary().to_digit_slice());
        }
        Ternary::new(digits).trim()
    }

    /// Converts the `DataTernary` into its fixed-length `Ternary` representation.
    ///
    /// This method iterates over all the `TritsChunk` instances in the `DataTernary` and
    /// extracts and combines their ternary digits into a single `Ternary` value.
    /// The resulting `Ternary` value will contain a fixed number of digits without trimming
    /// or removing leading zeroes.
    ///
    /// # Returns
    ///
    /// A `Ternary` value representing the combined fixed-length ternary digits of the `DataTernary`.
    ///
    /// # Example
    ///
    /// ```
    /// use balanced_ternary::{DataTernary, Ternary};
    ///
    /// let ternary = Ternary::from_dec(42);
    /// let data_ternary = DataTernary::from_ternary(ternary);
    /// let fixed_ternary = data_ternary.to_fixed_ternary();
    /// assert_eq!(fixed_ternary.to_dec(), 42); // When properly encoded
    /// ```
    pub fn to_fixed_ternary(&self) -> Ternary {
        let mut digits = Vec::new();
        for chunk in &self.chunks {
            digits.extend(chunk.to_digits());
        }
        Ternary::new(digits).trim()
    }

    /// Converts the `DataTernary` into a vector of ternary digits.
    ///
    /// This method first converts the `DataTernary` structure into its `Ternary` representation,
    /// trims any leading zeroes, and then returns the sequence of ternary digits as a `Vec<Digit>`.
    ///
    /// # Returns
    ///
    /// A `Vec<Digit>` containing the ternary digits that represent the `DataTernary` value.
    ///
    /// # Example
    ///
    /// ```
    /// use balanced_ternary::{DataTernary, Digit, Ternary};
    ///
    /// let ternary = Ternary::from_dec(42);
    /// let data_ternary = DataTernary::from_ternary(ternary);
    /// let digits = data_ternary.to_digits();
    /// assert_eq!(digits, vec![Digit::Pos, Digit::Neg, Digit::Neg, Digit::Neg, Digit::Zero]);
    /// ```
    pub fn to_digits(&self) -> Vec<Digit> {
        self.to_ternary().trim().to_digit_slice().to_vec()
    }

    /// Converts a decimal number into a `DataTernary` structure.
    ///
    /// This method takes a signed 64-bit integer as input and converts it into a
    /// `Ternary` representation, which is then stored in the compact `DataTernary`
    /// structure. The conversion ensures that the ternary representation uses
    /// fixed-length chunks for efficient storage.
    ///
    /// # Arguments
    ///
    /// * `from` - A signed 64-bit integer value to be converted into `DataTernary`.
    ///
    /// # Returns
    ///
    /// A `DataTernary` instance that represents the given decimal number.
    ///
    /// # Example
    ///
    /// ```
    /// use balanced_ternary::{DataTernary};
    ///
    /// let data_ternary = DataTernary::from_dec(42);
    /// assert_eq!(data_ternary.to_dec(), 42);
    /// ```
    pub fn from_dec(from: i64) -> Self {
        Self::from_ternary(Ternary::from_dec(from))
    }

    /// Converts a `DataTernary` into its decimal representation.
    ///
    /// This method reconstructs the ternary value represented by the `DataTernary`
    /// structure and converts it into the corresponding signed 64-bit decimal integer.
    ///
    /// # Returns
    ///
    /// A signed 64-bit integer (`i64`) representing the decimal equivalent of the
    /// `DataTernary` structure.
    ///
    /// # Example
    ///
    /// ```
    /// use balanced_ternary::{DataTernary};
    ///
    /// let data_ternary = DataTernary::from_dec(42);
    /// let decimal = data_ternary.to_dec();
    /// assert_eq!(decimal, 42);
    /// ```
    pub fn to_dec(&self) -> i64 {
        self.to_ternary().to_dec()
    }
}

impl Display for DataTernary {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for chunk in &self.chunks {
            write!(f, "{}", chunk.to_fixed_ternary())?;
        }
        Ok(())
    }
}

/// A struct to store 40 ternary digits (~63.398 bits) value into one `i64`.
pub struct Big(i64);

impl Big {
    pub fn from_dec(from: i64) -> Self {
        Self(from)
    }
    pub fn to_dec(&self) -> i64 {
        self.0
    }
    pub fn from_ternary(ternary: Ternary) -> Self {
        Self(ternary.to_dec())
    }
    pub fn to_ternary(&self) -> Ternary {
        Ternary::from_dec(self.0).with_length(40)
    }
}

impl DigitOperate for Big {
    fn to_digits(&self) -> Vec<Digit> {
        self.to_ternary().to_digits()
    }

    fn digit(&self, index: usize) -> Option<Digit> {
        self.to_ternary().digit(index)
    }

    fn each(&self, f: impl Fn(Digit) -> Digit) -> Self
    where
        Self: Sized,
    {
        Self(self.to_ternary().each(f).to_dec())
    }

    fn each_with(&self, f: impl Fn(Digit, Digit) -> Digit, other: Digit) -> Self
    where
        Self: Sized,
    {
        Self(self.to_ternary().each_with(f, other).to_dec())
    }

    fn each_zip(&self, f: impl Fn(Digit, Digit) -> Digit, other: Self) -> Self
    where
        Self: Sized,
    {
        Self(self.to_ternary().each_zip(f, other.to_ternary()).to_dec())
    }

    fn each_zip_carry(&self, f: impl Fn(Digit, Digit, Digit) -> (Digit, Digit), other: Self) -> Self
    where
        Self: Sized,
    {
        Self(
            self.to_ternary()
                .each_zip_carry(f, other.to_ternary())
                .to_dec(),
        )
    }
}
