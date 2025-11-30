//! Caching for infinite arrays to enable mutability

use std::collections::HashMap;
use crate::arrays::InfiniteArray;

/// A cached infinite array that stores computed values and allows mutation
pub struct CachedArray<T, A> {
    base: A,
    cache: HashMap<usize, T>,
}

impl<T, A> CachedArray<T, A>
where
    T: Clone,
    A: InfiniteArray<T>,
{
    /// Create a new cached array from a base infinite array
    pub fn new(base: A) -> Self {
        Self {
            base,
            cache: HashMap::new(),
        }
    }

    /// Get the value at the given index, using cache if available
    pub fn get(&self, index: usize) -> T {
        self.cache.get(&index)
            .cloned()
            .unwrap_or_else(|| self.base.get(index))
    }

    /// Set the value at the given index
    pub fn set(&mut self, index: usize, value: T) {
        self.cache.insert(index, value);
    }

    /// Get a mutable reference to the cached value, computing it if necessary
    pub fn get_mut(&mut self, index: usize) -> &mut T
    where
        T: Default,
    {
        self.cache.entry(index)
            .or_insert_with(|| self.base.get(index))
    }

    /// Create an iterator over the cached array
    pub fn iter(&self) -> CachedArrayIter<'_, T, A> {
        CachedArrayIter {
            cached: self,
            index: 0,
        }
    }

    /// Clear the cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// Get the number of cached entries
    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }
}

/// Iterator over a cached array
pub struct CachedArrayIter<'a, T, A> {
    cached: &'a CachedArray<T, A>,
    index: usize,
}

impl<'a, T, A> Iterator for CachedArrayIter<'a, T, A>
where
    T: Clone,
    A: InfiniteArray<T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = Some(self.cached.get(self.index));
        self.index += 1;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arrays::Ones;

    #[test]
    fn test_cached_array() {
        let ones = Ones::new();
        let mut cached = CachedArray::new(ones);
        
        assert_eq!(cached.get(0), 1.0);
        assert_eq!(cached.get(1), 1.0);
        
        cached.set(0, 3.0);
        assert_eq!(cached.get(0), 3.0);
        assert_eq!(cached.get(1), 1.0);
    }

    #[test]
    fn test_cached_array_iter() {
        let ones = Ones::new();
        let mut cached = CachedArray::new(ones);
        cached.set(0, 5.0);
        
        let mut iter = cached.iter();
        assert_eq!(iter.next(), Some(5.0));
        assert_eq!(iter.next(), Some(1.0));
        assert_eq!(iter.next(), Some(1.0));
    }
}

