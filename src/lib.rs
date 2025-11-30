//! # InfiniteArrays
//!
//! A Rust crate for representing arrays with infinite dimension sizes.
//! Infinite arrays are by necessity lazy, and support operations like
//! indexing, iteration, and various mathematical operations.
//!
//! ## Examples
//!
//! ```rust
//! use infinite_arrays::*;
//!
//! // Create an infinite vector of ones
//! let ones = Ones::new();
//! assert_eq!(ones.get(0), 1.0);
//! assert_eq!(ones.get(100), 1.0);
//!
//! // Create an infinite range
//! let range = OneToInf::new();
//! assert_eq!(range.get(0), 1);
//! assert_eq!(range.get(1), 2);
//!
//! // Cumulative sum
//! let cumsum_result = cumsum(ones);
//! assert_eq!(cumsum_result.get(0), 1.0);
//! assert_eq!(cumsum_result.get(1), 2.0);
//! ```

pub mod ranges;
pub mod arrays;
pub mod operations;
pub mod cache;

pub use ranges::{OneToInf, InfUnitRange, InfStepRange};
pub use arrays::{Ones, Zeros, InfiniteArray, InfiniteVector, InfiniteArrayFromFn};
pub use operations::{cumsum, broadcast, add_scalar, mul_scalar, add_arrays, sub_arrays, mul_arrays, div_arrays};
pub use cache::CachedArray;

/// The infinity symbol constant
pub const INFINITY: usize = usize::MAX;

/// Trait for types that can represent infinity
pub trait Infinite: Sized {
    fn infinity() -> Self;
}

impl Infinite for usize {
    fn infinity() -> Self {
        usize::MAX
    }
}

impl Infinite for isize {
    fn infinity() -> Self {
        isize::MAX
    }
}

impl Infinite for f64 {
    fn infinity() -> Self {
        f64::INFINITY
    }
}

impl Infinite for f32 {
    fn infinity() -> Self {
        f32::INFINITY
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ones() {
        let ones = Ones::new();
        assert_eq!(ones.get(0), 1.0);
        assert_eq!(ones.get(100), 1.0);
    }

    #[test]
    fn test_range() {
        let range = OneToInf::new();
        assert_eq!(range.get(0), 1);
        assert_eq!(range.get(1), 2);
        assert_eq!(range.get(99), 100);
    }
}

