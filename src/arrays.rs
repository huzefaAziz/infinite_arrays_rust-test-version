//! Infinite array types

use num_traits::{One, Zero};

/// Trait for infinite arrays that can be indexed
pub trait InfiniteArray<T> {
    /// Get the value at the given index
    fn get(&self, index: usize) -> T;
    
    /// Create an iterator over the array
    fn iter(&self) -> Box<dyn Iterator<Item = T> + '_>;
    
    /// Get the length (infinity for infinite arrays)
    fn len(&self) -> Option<usize> {
        None
    }
}

/// Trait for infinite vectors (1D arrays)
pub trait InfiniteVector<T>: InfiniteArray<T> {}

/// An infinite array filled with ones
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ones<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Ones<T> {
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T> Default for Ones<T> {
    fn default() -> Self {
        Self::new()
    }
}


impl<T> InfiniteArray<T> for Ones<T>
where
    T: One + Copy,
{
    fn get(&self, _index: usize) -> T {
        T::one()
    }

    fn iter(&self) -> Box<dyn Iterator<Item = T> + '_> {
        Box::new(OnesIter {
            value: T::one(),
        })
    }
}

impl<T> InfiniteVector<T> for Ones<T> where T: One + Copy {}

/// Iterator over Ones
struct OnesIter<T> {
    value: T,
}

impl<T> Iterator for OnesIter<T>
where
    T: Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.value)
    }
}

/// An infinite array filled with zeros
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Zeros<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Zeros<T> {
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T> Default for Zeros<T> {
    fn default() -> Self {
        Self::new()
    }
}


impl<T> InfiniteArray<T> for Zeros<T>
where
    T: Zero + Copy,
{
    fn get(&self, _index: usize) -> T {
        T::zero()
    }

    fn iter(&self) -> Box<dyn Iterator<Item = T> + '_> {
        Box::new(ZerosIter {
            value: T::zero(),
        })
    }
}

impl<T> InfiniteVector<T> for Zeros<T> where T: Zero + Copy {}

/// Iterator over Zeros
struct ZerosIter<T> {
    value: T,
}

impl<T> Iterator for ZerosIter<T>
where
    T: Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.value)
    }
}

/// An infinite array from a function
#[derive(Clone)]
pub struct InfiniteArrayFromFn<F, T> {
    f: F,
    _phantom: std::marker::PhantomData<T>,
}

impl<F, T> InfiniteArrayFromFn<F, T>
where
    F: Fn(usize) -> T,
{
    pub fn new(f: F) -> Self {
        Self {
            f,
            _phantom: std::marker::PhantomData,
        }
    }
}


impl<F, T> InfiniteArray<T> for InfiniteArrayFromFn<F, T>
where
    F: Fn(usize) -> T,
{
    fn get(&self, index: usize) -> T {
        (self.f)(index)
    }

    fn iter(&self) -> Box<dyn Iterator<Item = T> + '_> {
        // We can't clone the function, so we create a new iterator that borrows self
        Box::new((0..).map(move |i| (self.f)(i)))
    }
}

impl<F, T> InfiniteVector<T> for InfiniteArrayFromFn<F, T> where F: Fn(usize) -> T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ones() {
        let ones = Ones::new();
        assert_eq!(ones.get(0), 1.0);
        assert_eq!(ones.get(100), 1.0);
        
        let mut iter = ones.iter();
        assert_eq!(iter.next(), Some(1.0));
        assert_eq!(iter.next(), Some(1.0));
        assert_eq!(iter.next(), Some(1.0));
    }

    #[test]
    fn test_zeros() {
        let zeros = Zeros::new();
        assert_eq!(zeros.get(0), 0.0);
        assert_eq!(zeros.get(100), 0.0);
    }

    #[test]
    fn test_infinite_array_from_fn() {
        let arr = InfiniteArrayFromFn::new(|i| i * 2);
        assert_eq!(arr.get(0), 0);
        assert_eq!(arr.get(1), 2);
        assert_eq!(arr.get(5), 10);
    }
}

