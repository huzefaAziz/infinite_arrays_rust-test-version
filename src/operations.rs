//! Operations on infinite arrays

use crate::arrays::{InfiniteArray, InfiniteArrayFromFn};
use num_traits::Zero;
use std::ops::{Add, Sub, Mul, Div};

/// Cumulative sum of an infinite array
pub fn cumsum<T, A>(arr: A) -> InfiniteArrayFromFn<impl Fn(usize) -> T, T>
where
    T: Zero + Add<Output = T> + Copy,
    A: InfiniteArray<T> + Clone,
{
    InfiniteArrayFromFn::new(move |i| {
        // Clone the array for each cumulative sum calculation
        // This is necessary because we need to access previous elements
        let arr_clone = arr.clone();
        (0..=i).fold(T::zero(), |acc, idx| acc + arr_clone.get(idx))
    })
}

/// Broadcast a function over an infinite array
pub fn broadcast<F, TIn, TOut, A>(arr: A, f: F) -> InfiniteArrayFromFn<impl Fn(usize) -> TOut, TOut>
where
    F: Fn(TIn) -> TOut,
    A: InfiniteArray<TIn> + Clone,
    TIn: Clone,
    TOut: Clone,
{
    InfiniteArrayFromFn::new(move |i| {
        let arr_clone = arr.clone();
        f(arr_clone.get(i))
    })
}

/// Element-wise addition of two infinite arrays
pub fn add_arrays<T, A, B>(a: A, b: B) -> InfiniteArrayFromFn<impl Fn(usize) -> T, T>
where
    T: Add<Output = T> + Copy,
    A: InfiniteArray<T> + Clone,
    B: InfiniteArray<T> + Clone,
{
    InfiniteArrayFromFn::new(move |i| {
        let a_clone = a.clone();
        let b_clone = b.clone();
        a_clone.get(i) + b_clone.get(i)
    })
}

/// Element-wise subtraction of two infinite arrays
pub fn sub_arrays<T, A, B>(a: A, b: B) -> InfiniteArrayFromFn<impl Fn(usize) -> T, T>
where
    T: Sub<Output = T> + Copy,
    A: InfiniteArray<T> + Clone,
    B: InfiniteArray<T> + Clone,
{
    InfiniteArrayFromFn::new(move |i| {
        let a_clone = a.clone();
        let b_clone = b.clone();
        a_clone.get(i) - b_clone.get(i)
    })
}

/// Element-wise multiplication of two infinite arrays
pub fn mul_arrays<T, A, B>(a: A, b: B) -> InfiniteArrayFromFn<impl Fn(usize) -> T, T>
where
    T: Mul<Output = T> + Copy,
    A: InfiniteArray<T> + Clone,
    B: InfiniteArray<T> + Clone,
{
    InfiniteArrayFromFn::new(move |i| {
        let a_clone = a.clone();
        let b_clone = b.clone();
        a_clone.get(i) * b_clone.get(i)
    })
}

/// Element-wise division of two infinite arrays
pub fn div_arrays<T, A, B>(a: A, b: B) -> InfiniteArrayFromFn<impl Fn(usize) -> T, T>
where
    T: Div<Output = T> + Copy,
    A: InfiniteArray<T> + Clone,
    B: InfiniteArray<T> + Clone,
{
    InfiniteArrayFromFn::new(move |i| {
        let a_clone = a.clone();
        let b_clone = b.clone();
        a_clone.get(i) / b_clone.get(i)
    })
}

/// Scalar addition
pub fn add_scalar<T, A>(arr: A, scalar: T) -> InfiniteArrayFromFn<impl Fn(usize) -> T, T>
where
    T: Add<Output = T> + Copy,
    A: InfiniteArray<T> + Clone,
{
    InfiniteArrayFromFn::new(move |i| {
        let arr_clone = arr.clone();
        arr_clone.get(i) + scalar
    })
}

/// Scalar multiplication
pub fn mul_scalar<T, A>(arr: A, scalar: T) -> InfiniteArrayFromFn<impl Fn(usize) -> T, T>
where
    T: Mul<Output = T> + Copy,
    A: InfiniteArray<T> + Clone,
{
    InfiniteArrayFromFn::new(move |i| {
        let arr_clone = arr.clone();
        arr_clone.get(i) * scalar
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arrays::Ones;

    #[test]
    fn test_cumsum() {
        let ones = Ones::new();
        let cumsum_result = cumsum(ones);
        
        assert_eq!(cumsum_result.get(0), 1.0);
        assert_eq!(cumsum_result.get(1), 2.0);
        assert_eq!(cumsum_result.get(2), 3.0);
        assert_eq!(cumsum_result.get(9), 10.0);
    }

    #[test]
    fn test_broadcast() {
        let ones = Ones::new();
        let doubled = broadcast(ones, |x| x * 2.0);
        
        assert_eq!(doubled.get(0), 2.0);
        assert_eq!(doubled.get(100), 2.0);
    }

    #[test]
    fn test_add_scalar() {
        let ones = Ones::new();
        let result = add_scalar(ones, 2.0);
        
        assert_eq!(result.get(0), 3.0);
        assert_eq!(result.get(100), 3.0);
    }
}

