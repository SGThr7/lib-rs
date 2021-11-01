#[codesnip::entry("HashMultiSet")]
pub use hash_multiset::HashMultiSet;

#[codesnip::entry("HashMultiSet")]
pub mod hash_multiset {
    use core::{
        borrow::Borrow,
        fmt::{Debug, Formatter, Result},
        hash::{BuildHasher, Hash},
        iter::{FromIterator, FusedIterator},
    };
    use std::collections::{
        hash_map::{self, RandomState},
        HashMap,
    };

    pub struct HashMultiSet<T, S = RandomState> {
        len: usize,
        counter: HashMap<T, usize, S>,
    }

    impl<T, S> HashMultiSet<T, S> {
        pub fn len(&self) -> usize {
            self.len
        }

        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }

        pub fn clear(&mut self) {
            self.len = 0;
            self.counter.clear();
        }

        pub fn iter(&self) -> Iter<'_, T> {
            Iter {
                iter: self.counter.iter(),
                peek: None,
                peek_count: 0,
            }
        }
    }

    impl<T, S> HashMultiSet<T, S>
    where
        T: Eq + Hash,
        S: BuildHasher,
    {
        pub fn new() -> Self
        where
            S: Default,
        {
            Default::default()
        }

        pub fn insert(&mut self, value: T) {
            self.insert_times(value, 1)
        }

        pub fn insert_times(&mut self, value: T, count: usize) {
            self.len += count;
            *self.counter.entry(value).or_insert(0) += count
        }

        pub fn remove<Q>(&mut self, value: &Q) -> bool
        where
            T: Borrow<Q>,
            Q: Eq + Hash,
        {
            self.remove_times(value, 1) > 0
        }

        pub fn remove_times<Q>(&mut self, value: &Q, count: usize) -> usize
        where
            T: Borrow<Q>,
            Q: Eq + Hash,
        {
            if let Some(c) = self.counter.get(value) {
                if *c > count {
                    self.len -= count;
                    *self.counter.get_mut(value).unwrap() -= count;
                    count
                } else {
                    let t = self.counter.remove(value).unwrap();
                    self.len -= t;
                    t
                }
            } else {
                0
            }
        }

        pub fn count<Q>(&self, value: &Q) -> Option<usize>
        where
            T: Borrow<Q>,
            Q: Eq + Hash,
        {
            self.counter.get(value).copied()
        }
    }

    impl<T: Clone + Debug, S> Debug for HashMultiSet<T, S> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "{{")?;
            f.debug_set().entries(self.iter()).finish()?;
            write!(f, "}}")?;
            Ok(())
        }
    }

    impl<T, S> Default for HashMultiSet<T, S>
    where
        T: Eq + Hash,
        S: Default + BuildHasher,
    {
        fn default() -> Self {
            Self {
                counter: Default::default(),
                len: Default::default(),
            }
        }
    }

    impl<T, S> PartialEq for HashMultiSet<T, S>
    where
        T: Eq + Hash,
        S: BuildHasher,
    {
        fn eq(&self, other: &Self) -> bool {
            self.len == other.len && self.counter == other.counter
        }
    }
    impl<T, S> Eq for HashMultiSet<T, S>
    where
        T: Eq + Hash,
        S: BuildHasher,
    {
    }

    impl<'a, T, S> IntoIterator for &'a HashMultiSet<T, S> {
        type Item = &'a T;
        type IntoIter = Iter<'a, T>;
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }

    impl<T> FromIterator<T> for HashMultiSet<T>
    where
        T: Eq + Hash,
    {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            let mut ret = Self::new();
            for value in iter {
                ret.insert(value);
            }
            ret
        }
    }

    #[derive(Debug)]
    pub struct Iter<'a, T> {
        iter: hash_map::Iter<'a, T, usize>,
        peek: Option<(&'a T, &'a usize)>,
        peek_count: usize,
    }

    #[derive(Debug)]
    pub struct IntoIter<T> {
        iter: hash_map::IntoIter<T, usize>,
        peek: Option<(T, usize)>,
        peek_count: usize,
    }

    impl<'a, T> Iterator for Iter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            if self.peek.is_none() {
                self.peek = self.iter.next();
            }
            if let Some((value, count)) = self.peek {
                self.peek_count += 1;
                if &self.peek_count >= count {
                    self.peek = None;
                    self.peek_count = 0;
                }
                Some(value)
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            let (lower, _) = self.iter.size_hint();
            (lower, None)
        }

        fn count(self) -> usize
        where
            Self: Sized,
        {
            self.iter.fold(0, |acc, (_, count)| acc + count) - self.peek_count
        }
    }

    impl<T> FusedIterator for Iter<'_, T> {}

    impl<T> Clone for Iter<'_, T> {
        fn clone(&self) -> Self {
            Self {
                iter: self.iter.clone(),
                peek: self.peek.clone(),
                peek_count: self.peek_count,
            }
        }
    }

    impl<T: Clone> Iterator for IntoIter<T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            if self.peek.is_none() {
                self.peek = self.iter.next();
            }
            if let Some((value, count)) = self.peek.clone() {
                self.peek_count += 1;
                if self.peek_count >= count {
                    self.peek = None;
                    self.peek_count = 0;
                }
                Some(value)
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            let (lower, _) = self.iter.size_hint();
            (lower, None)
        }

        fn count(self) -> usize
        where
            Self: Sized,
        {
            self.iter.fold(0, |acc, (_, count)| acc + count) - self.peek_count
        }
    }

    impl<T: Clone> FusedIterator for IntoIter<T> {}

    #[cfg(test)]
    mod tests {
        use super::HashMultiSet;

        const NAPIER: [usize; 1001] = [
            2, 7, 1, 8, 2, 8, 1, 8, 2, 8, 4, 5, 9, 0, 4, 5, 2, 3, 5, 3, 6, 0, 2, 8, 7, 4, 7, 1, 3,
            5, 2, 6, 6, 2, 4, 9, 7, 7, 5, 7, 2, 4, 7, 0, 9, 3, 6, 9, 9, 9, 5, 9, 5, 7, 4, 9, 6, 6,
            9, 6, 7, 6, 2, 7, 7, 2, 4, 0, 7, 6, 6, 3, 0, 3, 5, 3, 5, 4, 7, 5, 9, 4, 5, 7, 1, 3, 8,
            2, 1, 7, 8, 5, 2, 5, 1, 6, 6, 4, 2, 7, 4, 2, 7, 4, 6, 6, 3, 9, 1, 9, 3, 2, 0, 0, 3, 0,
            5, 9, 9, 2, 1, 8, 1, 7, 4, 1, 3, 5, 9, 6, 6, 2, 9, 0, 4, 3, 5, 7, 2, 9, 0, 0, 3, 3, 4,
            2, 9, 5, 2, 6, 0, 5, 9, 5, 6, 3, 0, 7, 3, 8, 1, 3, 2, 3, 2, 8, 6, 2, 7, 9, 4, 3, 4, 9,
            0, 7, 6, 3, 2, 3, 3, 8, 2, 9, 8, 8, 0, 7, 5, 3, 1, 9, 5, 2, 5, 1, 0, 1, 9, 0, 1, 1, 5,
            7, 3, 8, 3, 4, 1, 8, 7, 9, 3, 0, 7, 0, 2, 1, 5, 4, 0, 8, 9, 1, 4, 9, 9, 3, 4, 8, 8, 4,
            1, 6, 7, 5, 0, 9, 2, 4, 4, 7, 6, 1, 4, 6, 0, 6, 6, 8, 0, 8, 2, 2, 6, 4, 8, 0, 0, 1, 6,
            8, 4, 7, 7, 4, 1, 1, 8, 5, 3, 7, 4, 2, 3, 4, 5, 4, 4, 2, 4, 3, 7, 1, 0, 7, 5, 3, 9, 0,
            7, 7, 7, 4, 4, 9, 9, 2, 0, 6, 9, 5, 5, 1, 7, 0, 2, 7, 6, 1, 8, 3, 8, 6, 0, 6, 2, 6, 1,
            3, 3, 1, 3, 8, 4, 5, 8, 3, 0, 0, 0, 7, 5, 2, 0, 4, 4, 9, 3, 3, 8, 2, 6, 5, 6, 0, 2, 9,
            7, 6, 0, 6, 7, 3, 7, 1, 1, 3, 2, 0, 0, 7, 0, 9, 3, 2, 8, 7, 0, 9, 1, 2, 7, 4, 4, 3, 7,
            4, 7, 0, 4, 7, 2, 3, 0, 6, 9, 6, 9, 7, 7, 2, 0, 9, 3, 1, 0, 1, 4, 1, 6, 9, 2, 8, 3, 6,
            8, 1, 9, 0, 2, 5, 5, 1, 5, 1, 0, 8, 6, 5, 7, 4, 6, 3, 7, 7, 2, 1, 1, 1, 2, 5, 2, 3, 8,
            9, 7, 8, 4, 4, 2, 5, 0, 5, 6, 9, 5, 3, 6, 9, 6, 7, 7, 0, 7, 8, 5, 4, 4, 9, 9, 6, 9, 9,
            6, 7, 9, 4, 6, 8, 6, 4, 4, 5, 4, 9, 0, 5, 9, 8, 7, 9, 3, 1, 6, 3, 6, 8, 8, 9, 2, 3, 0,
            0, 9, 8, 7, 9, 3, 1, 2, 7, 7, 3, 6, 1, 7, 8, 2, 1, 5, 4, 2, 4, 9, 9, 9, 2, 2, 9, 5, 7,
            6, 3, 5, 1, 4, 8, 2, 2, 0, 8, 2, 6, 9, 8, 9, 5, 1, 9, 3, 6, 6, 8, 0, 3, 3, 1, 8, 2, 5,
            2, 8, 8, 6, 9, 3, 9, 8, 4, 9, 6, 4, 6, 5, 1, 0, 5, 8, 2, 0, 9, 3, 9, 2, 3, 9, 8, 2, 9,
            4, 8, 8, 7, 9, 3, 3, 2, 0, 3, 6, 2, 5, 0, 9, 4, 4, 3, 1, 1, 7, 3, 0, 1, 2, 3, 8, 1, 9,
            7, 0, 6, 8, 4, 1, 6, 1, 4, 0, 3, 9, 7, 0, 1, 9, 8, 3, 7, 6, 7, 9, 3, 2, 0, 6, 8, 3, 2,
            8, 2, 3, 7, 6, 4, 6, 4, 8, 0, 4, 2, 9, 5, 3, 1, 1, 8, 0, 2, 3, 2, 8, 7, 8, 2, 5, 0, 9,
            8, 1, 9, 4, 5, 5, 8, 1, 5, 3, 0, 1, 7, 5, 6, 7, 1, 7, 3, 6, 1, 3, 3, 2, 0, 6, 9, 8, 1,
            1, 2, 5, 0, 9, 9, 6, 1, 8, 1, 8, 8, 1, 5, 9, 3, 0, 4, 1, 6, 9, 0, 3, 5, 1, 5, 9, 8, 8,
            8, 8, 5, 1, 9, 3, 4, 5, 8, 0, 7, 2, 7, 3, 8, 6, 6, 7, 3, 8, 5, 8, 9, 4, 2, 2, 8, 7, 9,
            2, 2, 8, 4, 9, 9, 8, 9, 2, 0, 8, 6, 8, 0, 5, 8, 2, 5, 7, 4, 9, 2, 7, 9, 6, 1, 0, 4, 8,
            4, 1, 9, 8, 4, 4, 4, 3, 6, 3, 4, 6, 3, 2, 4, 4, 9, 6, 8, 4, 8, 7, 5, 6, 0, 2, 3, 3, 6,
            2, 4, 8, 2, 7, 0, 4, 1, 9, 7, 8, 6, 2, 3, 2, 0, 9, 0, 0, 2, 1, 6, 0, 9, 9, 0, 2, 3, 5,
            3, 0, 4, 3, 6, 9, 9, 4, 1, 8, 4, 9, 1, 4, 6, 3, 1, 4, 0, 9, 3, 4, 3, 1, 7, 3, 8, 1, 4,
            3, 6, 4, 0, 5, 4, 6, 2, 5, 3, 1, 5, 2, 0, 9, 6, 1, 8, 3, 6, 9, 0, 8, 8, 8, 7, 0, 7, 0,
            1, 6, 7, 6, 8, 3, 9, 6, 4, 2, 4, 3, 7, 8, 1, 4, 0, 5, 9, 2, 7, 1, 4, 5, 6, 3, 5, 4, 9,
            0, 6, 1, 3, 0, 3, 1, 0, 7, 2, 0, 8, 5, 1, 0, 3, 8, 3, 7, 5, 0, 5, 1, 0, 1, 1, 5, 7, 4,
            7, 7, 0, 4, 1, 7, 1, 8, 9, 8, 6, 1, 0, 6, 8, 7, 3, 9, 6, 9, 6, 5, 5, 2, 1, 2, 6, 7, 1,
            5, 4, 6, 8, 8, 9, 5, 7, 0, 3, 5, 0, 3, 5, 4,
        ];

        #[test]
        fn debug_format() {
            let count = vec![100, 96, 98, 109, 100, 85, 99, 99, 103, 112];
            let mset = NAPIER.iter().collect::<HashMultiSet<_>>();
            let s = format!("{:?}", mset);
            let s_counter = {
                let s = &s[2..s.len() - 2];
                let s = s.split(",");
                let mut ret = vec![0; 10];
                s.map(|c| c.trim().parse::<usize>().unwrap())
                    .for_each(|x| ret[x] += 1);
                ret
            };
            assert_eq!(count, s_counter);
        }

        #[test]
        fn count() {
            let mset = NAPIER.iter().collect::<HashMultiSet<_>>();
            let count = vec![100, 96, 98, 109, 100, 85, 99, 99, 103, 112];
            for i in 0..10 {
                assert_eq!(count[i], mset.count(&i).unwrap_or(0), "i={}", i);
            }
        }
    }
}
