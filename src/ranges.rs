//! Infinite range types for indexing infinite arrays

use num_traits::One;

/// An infinite range starting from 1: 1, 2, 3, ...
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OneToInf<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> OneToInf<T> {
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T> Default for OneToInf<T> {
    fn default() -> Self {
        Self::new()
    }
}


impl<T> OneToInf<T>
where
    T: From<usize> + One + Copy,
{
    pub fn get(&self, index: usize) -> T {
        T::from(index + 1)
    }

    pub fn iter(&self) -> OneToInfIter<T> {
        OneToInfIter {
            current: T::one(),
            _phantom: std::marker::PhantomData,
        }
    }
}

/// Iterator over OneToInf
pub struct OneToInfIter<T> {
    current: T,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Iterator for OneToInfIter<T>
where
    T: std::ops::Add<Output = T> + One + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.current;
        self.current = self.current + T::one();
        Some(result)
    }
}

/// An infinite unit range starting from a given value
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InfUnitRange<T> {
    start: T,
}

impl<T> InfUnitRange<T> {
    pub fn new(start: T) -> Self {
        Self { start }
    }
}

impl<T> InfUnitRange<T>
where
    T: From<usize> + std::ops::Add<Output = T> + Copy,
{
    pub fn get(&self, index: usize) -> T {
        if index == 0 {
            self.start
        } else {
            self.start + T::from(index)
        }
    }

    pub fn iter(&self) -> InfUnitRangeIter<T> {
        InfUnitRangeIter {
            current: self.start,
        }
    }
}

/// Iterator over InfUnitRange
pub struct InfUnitRangeIter<T> {
    current: T,
}

impl<T> Iterator for InfUnitRangeIter<T>
where
    T: std::ops::Add<Output = T> + One + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.current;
        self.current = self.current + T::one();
        Some(result)
    }
}

/// An infinite step range: start, start+step, start+2*step, ...
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InfStepRange<T> {
    start: T,
    step: T,
}

impl<T> InfStepRange<T> {
    pub fn new(start: T, step: T) -> Self {
        Self { start, step }
    }
}

impl<T> InfStepRange<T>
where
    T: From<usize> + std::ops::Add<Output = T> + std::ops::Mul<Output = T> + Copy,
{
    pub fn get(&self, index: usize) -> T {
        self.start + self.step * T::from(index)
    }

    pub fn iter(&self) -> InfStepRangeIter<T> {
        InfStepRangeIter {
            current: self.start,
            step: self.step,
        }
    }
}

/// Iterator over InfStepRange
pub struct InfStepRangeIter<T> {
    current: T,
    step: T,
}

impl<T> Iterator for InfStepRangeIter<T>
where
    T: std::ops::Add<Output = T> + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.current;
        self.current = self.current + self.step;
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_to_inf() {
        let range = OneToInf::new();
        assert_eq!(range.get(0), 1);
        assert_eq!(range.get(1), 2);
        assert_eq!(range.get(99), 100);
    }

    #[test]
    fn test_one_to_inf_iter() {
        let range = OneToInf::new();
        let mut iter = range.iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
    }

    #[test]
    fn test_inf_unit_range() {
        let range = InfUnitRange::new(5);
        assert_eq!(range.get(0), 5);
        assert_eq!(range.get(1), 6);
        assert_eq!(range.get(10), 15);
    }

    #[test]
    fn test_inf_step_range() {
        let range = InfStepRange::new(0, 2);
        assert_eq!(range.get(0), 0);
        assert_eq!(range.get(1), 2);
        assert_eq!(range.get(2), 4);
    }
}

