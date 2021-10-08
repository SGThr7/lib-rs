#[codesnip::entry("BTreeMultiSet")]
pub mod btree_multiset {
    use core::{
        borrow::Borrow,
        fmt::Debug,
        hash::{BuildHasher, Hash},
        iter::FromIterator,
        iter::FusedIterator,
        mem::swap,
    };
    use std::collections::{btree_set, hash_map::RandomState, BTreeSet, HashMap};

    pub struct BTreeMultiSet<T, S = RandomState> {
        len: usize,
        tree: BTreeSet<T>,
        counter: HashMap<T, usize, S>,
    }

    impl<T, S> BTreeMultiSet<T, S>
    where
        T: Ord + Hash,
        S: Default + BuildHasher,
    {
        pub fn new() -> Self {
            Default::default()
        }
    }

    impl<T, S> BTreeMultiSet<T, S> {
        pub fn len(&self) -> usize {
            self.len
        }

        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }

        pub fn clear(&mut self)
        where
            T: Ord,
        {
            self.len = 0;
            self.tree.clear();
            self.counter.clear();
        }

        pub fn contains<Q>(&self, value: &Q) -> bool
        where
            T: Borrow<Q> + Ord,
            Q: Ord + ?Sized,
        {
            self.tree.contains(value)
        }

        pub fn get<Q>(&self, value: &Q) -> Option<&T>
        where
            T: Borrow<Q> + Ord,
            Q: Ord + ?Sized,
        {
            self.tree.get(value)
        }
    }

    impl<T, S> BTreeMultiSet<T, S>
    where
        S: BuildHasher,
    {
        pub fn count<Q>(&self, value: &Q) -> usize
        where
            T: Borrow<Q> + Eq + Hash,
            Q: Eq + Hash,
        {
            self.counter.get(value).copied().unwrap_or_default()
        }

        pub fn insert(&mut self, value: T) -> bool
        where
            T: Clone + Ord + Hash,
        {
            self.len += 1;
            *self.counter.entry(value.clone()).or_insert(0) += 1;
            self.tree.insert(value)
        }

        pub fn remove<Q>(&mut self, value: &Q) -> bool
        where
            T: Borrow<Q> + Ord + Hash,
            Q: Ord + Hash,
        {
            if let Some(count) = self.counter.get_mut(value) {
                *count -= 1;
                if *count == 0 {
                    self.tree.remove(value);
                    self.counter.remove(value);
                }
                true
            } else {
                false
            }
        }

        pub fn take<Q>(&mut self, value: &Q) -> Option<T>
        where
            T: Borrow<Q> + Ord + Clone + Hash,
            Q: Ord + Hash,
        {
            if let Some(count) = self.counter.get_mut(value) {
                *count -= 1;
                if *count == 0 {
                    self.counter.remove(value);
                    self.tree.take(value)
                } else {
                    self.tree.get(value).cloned()
                }
            } else {
                None
            }
        }

        pub fn iter(&self) -> Iter<'_, T, S>
        where
            T: Eq + Hash,
        {
            Iter {
                iter: self.tree.iter(),
                counter: &self.counter,
                peek: None,
                peek_count: 0,
            }
        }

        pub fn is_disjoint(&self, other: &BTreeMultiSet<T, S>) -> bool
        where
            T: Ord,
        {
            if self.tree.len() < other.tree.len() {
                self.tree.iter().all(|v| !other.contains(v))
            } else {
                other.tree.iter().all(|v| !self.contains(v))
            }
        }

        pub fn is_subset(&self, other: &BTreeMultiSet<T, S>) -> bool
        where
            T: Ord + Hash,
        {
            self.len() <= other.len()
                && self
                    .counter
                    .iter()
                    .all(|(value, count)| other.counter.get(value).map_or(false, |c| c >= count))
        }

        pub fn is_superset(&self, other: &BTreeMultiSet<T, S>) -> bool
        where
            T: Ord + Hash,
        {
            other.is_subset(self)
        }

        pub fn append(&mut self, other: &mut BTreeMultiSet<T, S>)
        where
            T: Ord + Hash,
        {
            if other.is_empty() {
                return;
            }

            if self.is_empty() {
                swap(self, other);
                return;
            }

            other
                .counter
                .drain()
                .for_each(|(v, c)| *self.counter.entry(v).or_insert(0) += c);
            self.tree.append(&mut other.tree);
            self.len += other.len;
            other.len = 0;
        }
    }

    impl<T, S> Default for BTreeMultiSet<T, S>
    where
        T: Ord + Hash,
        S: Default + BuildHasher,
    {
        fn default() -> Self {
            Self {
                len: Default::default(),
                tree: Default::default(),
                counter: Default::default(),
            }
        }
    }

    impl<T, S> Debug for BTreeMultiSet<T, S>
    where
        T: Debug + Eq + Hash,
        S: BuildHasher,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{{")?;
            f.debug_set().entries(self.iter()).finish()?;
            write!(f, "}}")?;
            Ok(())
        }
    }

    impl<T, S> PartialEq for BTreeMultiSet<T, S>
    where
        T: Eq + Hash,
        S: BuildHasher,
    {
        fn eq(&self, other: &Self) -> bool {
            self.len == other.len && self.counter == other.counter
        }
    }

    impl<T, S> Eq for BTreeMultiSet<T, S>
    where
        T: Eq + Hash,
        S: BuildHasher,
    {
    }

    impl<'a, T, S> IntoIterator for &'a BTreeMultiSet<T, S>
    where
        T: Eq + Hash,
        S: BuildHasher,
    {
        type Item = &'a T;

        type IntoIter = Iter<'a, T, S>;

        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }

    impl<T, S> IntoIterator for BTreeMultiSet<T, S>
    where
        T: Clone + Eq + Hash,
        S: BuildHasher,
    {
        type Item = T;

        type IntoIter = IntoIter<T, S>;

        fn into_iter(self) -> Self::IntoIter {
            IntoIter {
                iter: self.tree.into_iter(),
                counter: self.counter,
                peek: None,
                peek_count: 0,
            }
        }
    }

    impl<T, S> FromIterator<T> for BTreeMultiSet<T, S>
    where
        T: Clone + Ord + Hash,
        S: Default + BuildHasher,
    {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            let mut ret = Self::new();
            for value in iter {
                ret.insert(value);
            }
            ret
        }
    }

    pub struct Iter<'a, T, S> {
        iter: btree_set::Iter<'a, T>,
        counter: &'a HashMap<T, usize, S>,
        peek: Option<(&'a T, &'a usize)>,
        peek_count: usize,
    }

    pub struct IntoIter<T, S> {
        iter: btree_set::IntoIter<T>,
        counter: HashMap<T, usize, S>,
        peek: Option<(T, usize)>,
        peek_count: usize,
    }

    impl<'a, T, S> Iterator for Iter<'a, T, S>
    where
        T: Eq + Hash,
        S: BuildHasher,
    {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            if self.peek.is_none() {
                if let Some(next) = self.iter.next() {
                    self.peek = self.counter.get_key_value(next);
                }
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
    }

    impl<'a, T, S> FusedIterator for Iter<'a, T, S>
    where
        T: Eq + Hash,
        S: BuildHasher,
    {
    }

    impl<T, S> Iterator for IntoIter<T, S>
    where
        T: Clone + Eq + Hash,
        S: BuildHasher,
    {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            if self.peek.is_none() {
                if let Some(next) = self.iter.next() {
                    self.peek = self.counter.remove_entry(&next);
                }
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
    }

    #[cfg(test)]
    mod tests {
        use super::BTreeMultiSet;

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
        fn append() {
            let half = NAPIER.len() / 2;
            let mut head = NAPIER[..half].iter().collect::<BTreeMultiSet<_>>();
            let mut tail = NAPIER[half..].iter().collect::<BTreeMultiSet<_>>();
            let total_len = head.len() + tail.len();
            let total = NAPIER.iter().collect::<BTreeMultiSet<_>>();
            head.append(&mut tail);
            assert_eq!(head.len(), total_len);
            assert_eq!(tail.len(), 0);
            assert_eq!(head, total);
        }

        #[test]
        fn is_subset() {
            let sup = vec![1, 2, 3, 3, 4]
                .into_iter()
                .collect::<BTreeMultiSet<_>>();
            let mut set = BTreeMultiSet::new();

            assert!(set.is_subset(&sup));
            set.insert(2);
            assert!(set.is_subset(&sup));
            set.insert(3);
            assert!(set.is_subset(&sup));
            set.insert(3);
            assert!(set.is_subset(&sup));
            set.insert(3);
            assert!(!set.is_subset(&sup));
            set.remove(&3);
            assert!(set.is_subset(&sup));
            set.insert(3);
            assert!(!set.is_subset(&sup));
            set.remove(&3);
            assert!(set.is_subset(&sup));
            set.insert(5);
            assert!(!set.is_subset(&sup));
        }
    }
}
