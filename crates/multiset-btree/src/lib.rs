use std::{
    borrow::Borrow,
    collections::{
        btree_map::{IntoIter as BTreeMapIntoIter, Iter as BTreeMapIter, Range as BTreeMapRange},
        BTreeMap,
    },
    fmt::{self, Debug},
    hash::Hash,
    iter::{FromIterator, FusedIterator},
    mem,
    ops::{BitAnd, BitOr, BitXor, RangeBounds, Sub},
};

mod iter;
use iter::Iter;

mod into_iter;
use into_iter::IntoIter;

mod range;
use range::Range;

mod merge_iter;

mod symmetric_difference;
use symmetric_difference::SymmetricDifference;

mod union;
use union::Union;

mod intersection;
use intersection::Intersection;

mod difference;
use difference::Difference;

#[derive(Clone)]
pub struct BTreeMultiSet<T> {
    tree: BTreeMap<T, usize>,
    len: usize,
}

impl<T: Ord> BTreeMultiSet<T> {
    /// Makes a new, empty `BTreeMultiSet`.
    ///
    /// Does not allocate anything on its own.
    ///
    /// # Complexity
    ///
    /// | Time |
    /// | ---- |
    /// | O(1) |
    ///
    /// # Examples
    ///
    /// ```
    /// use multiset_btree::BTreeMultiSet;
    ///
    /// let mut set: BTreeMultiSet<i32> = BTreeMultiSet::new();
    /// ```
    pub fn new() -> Self {
        Self {
            tree: BTreeMap::new(),
            len: 0,
        }
    }

    /// Returns the number of elements in the set.
    ///
    /// # Complexity
    ///
    /// | Time |
    /// | ---- |
    /// | O(1) |
    ///
    /// # Examples
    ///
    /// ```
    /// use multiset_btree::BTreeMultiSet;
    ///
    /// let mut set = BTreeMultiSet::new();
    /// assert_eq!(set.len(), 0);
    /// set.insert(2);
    /// assert_eq!(set.len(), 1);
    /// set.insert(2);
    /// assert_eq!(set.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the set contains no elements.
    ///
    /// # Complexity
    ///
    /// | Time |
    /// | ---- |
    /// | O(1) |
    ///
    /// # Examples
    ///
    /// ```
    /// use multiset_btree::BTreeMultiSet;
    ///
    /// let mut set = BTreeMultiSet::new();
    /// assert!(set.is_empty());
    /// set.insert(2);
    /// assert!(!set.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the number of kinds of elements in the set.
    ///
    /// # Complexity
    ///
    /// | Time |
    /// | ---- |
    /// | O(1) |
    ///
    /// # Examples
    ///
    /// ```
    /// use multiset_btree::BTreeMultiSet;
    ///
    /// let mut set = BTreeMultiSet::new();
    /// assert_eq!(set.count_without_dup(), 0);
    /// set.insert(2);
    /// assert_eq!(set.count_without_dup(), 1);
    /// set.insert(2);
    /// assert_eq!(set.count_without_dup(), 1);
    /// set.insert(3);
    /// assert_eq!(set.count_without_dup(), 2);
    /// ```
    pub fn count_without_dup(&self) -> usize {
        self.tree.len()
    }

    /// Clear the set, removing all values.
    ///
    /// # Complexity
    ///
    /// | Time |
    /// | ---- |
    /// | O(1) |
    ///
    /// # Examples
    ///
    /// ```
    /// use multiset_btree::BTreeMultiSet;
    ///
    /// let mut set = vec![1, 2, 2, 3].into_iter().collect::<BTreeMultiSet<_>>();
    /// assert_eq!(set.len(), 4);
    /// set.clear();
    /// assert!(set.is_empty());
    /// ```
    pub fn clear(&mut self) {
        *self = Self::new();
    }

    /// Add a value to the set.
    ///
    /// Allow to add a duplicate value to the set.
    ///
    /// # Complexity
    ///
    /// N: `self.count_without_dup()`
    ///
    /// | Time     |
    /// | -------- |
    /// | O(log N) |
    ///
    /// # Examples
    ///
    /// ```
    /// use multiset_btree::BTreeMultiSet;
    ///
    /// let mut set = BTreeMultiSet::new();
    /// set.insert(1);
    /// set.insert(2);
    /// assert_eq!(set.len(), 2);
    /// set.insert(2);
    /// assert_eq!(set.len(), 3);
    /// ```
    pub fn insert(&mut self, value: T)
    where
        T: Ord,
    {
        let count = self.tree.entry(value).or_insert(0);
        *count += 1;
        self.len += 1;
    }

    /// Remove a value from the set.
    ///
    /// # Complexity
    ///
    /// N: `self.count_without_dup()`
    ///
    /// | Time     |
    /// | -------- |
    /// | O(log N) |
    ///
    /// # Examples
    ///
    /// ```
    /// use multiset_btree::BTreeMultiSet;
    ///
    /// let mut set = BTreeMultiSet::new();
    /// set.insert(2);
    /// set.insert(2);
    /// assert_eq!(set.len(), 2);
    /// set.remove(&2);
    /// assert_eq!(set.len(), 1);
    /// ```
    pub fn remove<Q>(&mut self, value: &Q)
    where
        T: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        if let Some(count) = self.tree.get_mut(value) {
            *count -= 1;
            self.len -= 1;
            if count == &0 {
                self.tree.remove(value);
            }
        }
    }

    /// Returns `true` if the set contains an element equal to the value.
    ///
    /// The value may be any borrowed from of the set's element type,
    /// but the ordering on the borrowed from *must* match the ordering on the element type.
    ///
    /// # Complexity
    ///
    /// N: `self.count_without_dup()`
    ///
    /// | Time     |
    /// | -------- |
    /// | O(log N) |
    ///
    /// # Examples
    ///
    /// ```
    /// use multiset_btree::BTreeMultiSet;
    ///
    /// let set = vec![1, 2, 2, 3].into_iter().collect::<BTreeMultiSet<_>>();
    /// assert_eq!(set.contains(&2), true);
    /// assert_eq!(set.contains(&4), false);
    /// ```
    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        T: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        self.tree.contains_key(value)
    }

    /// Returns a reference to one of the element in the set, if any, that is equal to the value.
    ///
    /// The value may be any borrowed from of the set's element type,
    /// but the ordering on the borrowed from *must* match the ordering on the element type.
    ///
    /// # Complexity
    ///
    /// N: `self.count_without_dup()`
    ///
    /// | Time     |
    /// | -------- |
    /// | O(log N) |
    ///
    /// # Examples
    ///
    /// ```
    /// use multiset_btree::BTreeMultiSet;
    ///
    /// let set = vec![1, 2, 2, 3].into_iter().collect::<BTreeMultiSet<_>>();
    /// assert_eq!(set.get(&2), Some(&2));
    /// assert_eq!(set.get(&3), Some(&3));
    /// assert_eq!(set.get(&4), None);
    /// ```
    pub fn get<Q>(&self, value: &Q) -> Option<&T>
    where
        T: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        self.tree.get_key_value(value).map(|(x, _)| x)
    }

    /// Returns the number of elements in the set equal to the value.
    ///
    /// # Complexity
    ///
    /// N: `self.len()`
    ///
    /// | Time     |
    /// | -------- |
    /// | O(log N) |
    ///
    /// # Examples
    ///
    /// ```
    /// use multiset_btree::BTreeMultiSet;
    ///
    /// let set = vec![1, 2, 2, 3].into_iter().collect::<BTreeMultiSet<_>>();
    /// assert_eq!(set.count(&2), 2);
    /// assert_eq!(set.count(&3), 1);
    /// assert_eq!(set.count(&4), 0);
    /// ```
    pub fn count<Q>(&self, value: &Q) -> usize
    where
        T: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        self.tree.get(value).copied().unwrap_or(0)
    }

    /// Visits the all elements in the set in ascending order.
    ///
    /// # Complexity
    ///
    /// N: `self.len()`
    ///
    /// | Iterator Size |
    /// | ------------- |
    /// | Θ(N)          |
    ///
    /// # Examples
    ///
    /// ```
    /// use multiset_btree::BTreeMultiSet;
    ///
    /// let set = vec![2, 3, 1, 2].into_iter().collect::<BTreeMultiSet<_>>();
    /// let mut set_iter = set.iter();
    /// assert_eq!(set_iter.next(), Some(&1));
    /// assert_eq!(set_iter.next(), Some(&2));
    /// assert_eq!(set_iter.next(), Some(&2));
    /// assert_eq!(set_iter.next(), Some(&3));
    /// assert_eq!(set_iter.next(), None);
    /// ```
    pub fn iter(&self) -> Iter<'_, T> {
        Iter::new(self)
    }

    /// Constructs a iterator over a sub-range of elements in the set.
    ///
    /// # Complexity
    ///
    /// N: `self.len()`
    ///
    /// | Iterator Size |
    /// | ------------- |
    /// | O(N)          |
    ///
    /// # Examples
    ///
    /// ```
    /// use multiset_btree::BTreeMultiSet;
    ///
    /// let set = vec![3, 5, 5, 8].into_iter().collect::<BTreeMultiSet<_>>();
    /// let range = set.range(4..).copied().collect::<Vec<_>>();
    /// assert_eq!(range, vec![5, 5, 8]);
    /// ```
    pub fn range<I, R>(&self, range: R) -> Range<'_, T>
    where
        T: Ord + Borrow<I>,
        I: Ord + ?Sized,
        R: RangeBounds<I>,
    {
        Range::new(self, range)
    }

    /// Removes and returns the element in the set, if any, that is equal to the value.
    ///
    /// The value may be any borrowed from of the set's element type,
    /// but the ordering on the borrowed from *must* match the ordering on the element type.
    ///
    /// # Complexity
    ///
    /// N: `self.count_without_dup()`
    ///
    /// | Time     |
    /// | -------- |
    /// | O(log N) |
    ///
    /// # Examples
    ///
    /// ```
    /// use multiset_btree::BTreeMultiSet;
    ///
    /// let mut set = vec![1, 2, 2, 3].into_iter().collect::<BTreeMultiSet<_>>();
    /// assert_eq!(set.take(&2), Some(2));
    /// assert_eq!(set.take(&2), Some(2));
    /// assert_eq!(set.take(&2), None);
    /// ```
    pub fn take<Q>(&mut self, value: &Q) -> Option<T>
    where
        T: Borrow<Q> + Ord + Clone,
        Q: Ord + ?Sized,
    {
        if let Some(ret) = self.get(value).cloned() {
            self.remove(value);
            Some(ret)
        } else {
            None
        }
    }

    /// Splits the collection into two at the value.
    /// Return a new collection with all elements greater than or equal to the value.
    ///
    /// # Complexity
    ///
    /// N: `self.count_without_dup()`
    ///
    /// | Time     |
    /// | -------- |
    /// | O(log N) |
    ///
    /// # Examples
    ///
    /// ```
    /// use multiset_btree::BTreeMultiSet;
    ///
    /// let mut a = vec![1, 2, 2, 3, 5, 5, 7].into_iter().collect::<BTreeMultiSet<_>>();
    /// let b = a.split_off(&3);
    ///
    /// assert_eq!(a.len(), 3);
    /// assert_eq!(b.len(), 4);
    ///
    /// assert_eq!(a.count(&1), 1);
    /// assert_eq!(a.count(&2), 2);
    ///
    /// assert_eq!(b.count(&3), 1);
    /// assert_eq!(b.count(&5), 2);
    /// assert_eq!(b.count(&7), 1);
    /// ```
    pub fn split_off<Q>(&mut self, value: &Q) -> BTreeMultiSet<T>
    where
        T: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        let ret = self.tree.split_off(value);
        let ret: BTreeMultiSet<T> = ret.into();
        self.len -= ret.len();
        ret
    }

    /// Moves all elements from `other` into `Self`, leaving `other` empty.
    ///
    /// # Complexity
    ///
    /// N: `self.count_without_dup()`
    /// M: `other.count_without_dup()`
    ///
    /// | Time         |
    /// | ------------ |
    /// | O(min(N, M)) |
    ///
    /// # Examples
    ///
    /// ```
    /// use multiset_btree::BTreeMultiSet;
    ///
    /// let mut a = vec![1, 2, 2, 3].into_iter().collect::<BTreeMultiSet<_>>();
    /// let mut b = vec![2, 3, 4, 4].into_iter().collect::<BTreeMultiSet<_>>();
    ///
    /// a.append(&mut b);
    ///
    /// assert_eq!(a.len(), 8);
    /// assert!(b.is_empty());
    ///
    /// assert_eq!(
    ///     a.into_iter().collect::<Vec<_>>(),
    ///     [1, 2, 2, 2, 3, 3, 4, 4]
    /// );
    /// ```
    pub fn append(&mut self, other: &mut BTreeMultiSet<T>)
    where
        T: Ord,
    {
        if self.tree.len() < other.tree.len() {
            mem::swap(self, other);
        }

        mem::take(other).tree.into_iter().for_each(|(x, count)| {
            *self.tree.entry(x).or_insert(0) += count;
            self.len += count;
        });
    }

    /// Visits the elements representing the symmetric difference,
    /// i.e., the elements that are in `Self` or `other` but not in both,
    /// in ascending order.
    ///
    /// # Complexity
    ///
    /// N: `self.len()`,
    /// M: `other.len()`,
    ///
    /// | Iterator Size |
    /// | ------------- |
    /// | O(N + M)      |
    ///
    /// # Examples
    ///
    /// ```
    /// use multiset_btree::BTreeMultiSet;
    ///
    /// let a = vec![1, 2, 2, 3].into_iter().collect::<BTreeMultiSet<_>>();
    /// let b = vec![2, 3, 4, 4].into_iter().collect::<BTreeMultiSet<_>>();
    ///
    /// let sym_diff = a.symmetric_difference(&b).cloned().collect::<Vec<_>>();
    /// assert_eq!(sym_diff, [1, 2, 4, 4]);
    pub fn symmetric_difference<'a>(
        &'a self,
        other: &'a BTreeMultiSet<T>,
    ) -> SymmetricDifference<'a, T>
    where
        T: Ord,
    {
        SymmetricDifference::new(self, other)
    }

    /// Visits the elements representing the union,
    /// i.e., the elements that are in `Self` or `other`,
    /// in ascending order.
    ///
    /// # Complexity
    ///
    /// N: `self.len()`,
    /// M: `other.len()`,
    ///
    /// | Iterator Size |
    /// | ------------- |
    /// | Θ(N + M)      |
    ///
    /// # Examples
    ///
    /// ```
    /// use multiset_btree::BTreeMultiSet;
    ///
    /// let a = vec![1, 2, 2, 3].into_iter().collect::<BTreeMultiSet<_>>();
    /// let b = vec![2, 3, 4, 4].into_iter().collect::<BTreeMultiSet<_>>();
    ///
    /// let union = a.union(&b).cloned().collect::<Vec<_>>();
    /// assert_eq!(union, [1, 2, 2, 2, 3, 3, 4, 4]);
    /// ```
    pub fn union<'a>(&'a self, other: &'a BTreeMultiSet<T>) -> Union<'a, T>
    where
        T: Ord,
    {
        Union::new(self, other)
    }

    /// Visits the elements representing the intersection,
    /// i.e., the elements that are in both `self` and `other`,
    /// in ascending order.
    ///
    /// # Complexity
    ///
    /// N: `self.len()`,
    /// M: `other.len()`
    ///
    /// | Iterator Size |
    /// | ------------- |
    /// | O(N + M)      |
    ///
    /// # Examples
    ///
    /// ```
    /// use multiset_btree::BTreeMultiSet;
    ///
    /// let a = vec![1, 2, 2, 3, 4].into_iter().collect::<BTreeMultiSet<_>>();
    /// let b = vec![2, 2, 2, 3, 3].into_iter().collect::<BTreeMultiSet<_>>();
    ///
    /// let intersection = a.intersection(&b).cloned().collect::<Vec<_>>();
    /// assert_eq!(intersection, [2, 2, 3]);
    /// ```
    pub fn intersection<'a>(&'a self, other: &'a BTreeMultiSet<T>) -> Intersection<'a, T> {
        Intersection::new(self, other)
    }

    /// Visits the element representing the difference,
    /// i.e., the elements that are in `self` but not in `other`,
    /// in ascending order.
    ///
    /// # Complexity
    ///
    /// N: `self.len()`,
    /// M: `other.len()`
    ///
    /// | Iterator Size |
    /// | ------------- |
    /// | O(N + M)      |
    ///
    /// # Examples
    ///
    /// ```
    /// use multiset_btree::BTreeMultiSet;
    ///
    /// let a = vec![1, 2, 2, 3, 3].into_iter().collect::<BTreeMultiSet<_>>();
    /// let b = vec![2, 2, 2, 3, 4].into_iter().collect::<BTreeMultiSet<_>>();
    ///
    /// let difference = a.difference(&b).cloned().collect::<Vec<_>>();
    /// assert_eq!(difference, [1, 3]);
    /// ```
    pub fn difference<'a>(&'a self, other: &'a BTreeMultiSet<T>) -> Difference<'a, T> {
        Difference::new(self, other)
    }

    /// Returns `true` if `self` has no elements in common with `other`.
    /// This is equivalent to checking for an empty intersection.
    ///
    /// # Complexity
    ///
    /// N: `self.len()`,
    /// M: `other.len()`
    ///
    /// The worst case is that `self` and `other` are disjoint multisets.
    ///
    /// The best case is that minimum elements of `self` and `other` are equal.
    ///
    /// | Time (Worst) | Time (Best) |
    /// | ------------ | ----------- |
    /// | O(N + M)     | O(1)        |
    ///
    /// # Examples
    ///
    /// ```
    /// use multiset_btree::BTreeMultiSet;
    ///
    /// let a = vec![1, 2, 2, 3].into_iter().collect::<BTreeMultiSet<_>>();
    /// let mut b = BTreeMultiSet::new();   
    ///
    /// assert!(a.is_disjoint(&b));
    /// b.insert(4);
    /// assert!(a.is_disjoint(&b));
    /// b.insert(1);
    /// assert!(!a.is_disjoint(&b));
    /// ```
    pub fn is_disjoint(&self, other: &BTreeMultiSet<T>) -> bool
    where
        T: Ord,
    {
        self.intersection(other).next().is_none()
    }

    /// Returns `true` if `other` has all elements in `self`.
    ///
    /// # Complexity
    ///
    /// M: `other.len()`,
    ///
    /// The worst case is that `self` is subset of `other`.
    ///
    /// The best case is that the minimum element of `self` is not contained in `other`.
    ///
    /// | Time (Worst) | Time (Best) |
    /// | ------------ | ----------- |
    /// | O(M)         | O(1)        |
    ///
    /// # Examples
    ///
    /// ```
    /// use multiset_btree::BTreeMultiSet;
    ///
    /// let sup = vec![1, 2, 2, 3].into_iter().collect::<BTreeMultiSet<_>>();
    /// let mut set = BTreeMultiSet::new();
    ///
    /// assert!(set.is_subset(&sup));
    /// set.insert(1);
    /// assert!(set.is_subset(&sup));
    /// set.insert(2);
    /// assert!(set.is_subset(&sup));
    /// set.insert(2);
    /// assert!(set.is_subset(&sup));
    /// set.insert(2);
    /// assert!(!set.is_subset(&sup));
    /// ```
    pub fn is_subset(&self, other: &BTreeMultiSet<T>) -> bool
    where
        T: Ord,
    {
        self.len() <= other.len() && self.difference(other).next().is_none()
    }

    /// Returns `true` if `self` has all elements in `other`.
    /// This is equivalent to `other.is_subset(&self)`.
    ///
    /// # Complexity
    ///
    /// N: `self.len()`
    ///
    /// The worst case is that `self` is superset of `other`.
    ///
    /// The best case is that the minimum element of `self` is not contained in `other`.
    ///
    /// | Time (Worst) | Time (Best) |
    /// | ------------ | ----------- |
    /// | O(N)         | O(1)        |
    ///
    /// # Examples
    ///
    /// ```
    /// use multiset_btree::BTreeMultiSet;
    ///
    /// let sub = vec![1, 2, 2].into_iter().collect::<BTreeMultiSet<_>>();
    /// let mut set = BTreeMultiSet::new();
    ///
    /// assert!(!set.is_superset(&sub));
    /// set.insert(1);
    /// set.insert(2);
    /// set.insert(2);
    /// assert!(set.is_superset(&sub));
    /// set.insert(2);
    /// set.insert(3);
    /// assert!(set.is_superset(&sub));
    /// ```
    pub fn is_superset(&self, other: &BTreeMultiSet<T>) -> bool
    where
        T: Ord,
    {
        other.is_subset(self)
    }

    // WARNING: After 1.53.0
    // /// Retains only the elements specified by the predicate.
    // ///
    // /// In other words, remove all elements `e` such that `f(&e)` returns `false`.
    // /// The elements are visited in ascending order.
    // ///
    // /// # Complexity
    // ///
    // /// N: `self.len()`
    // ///
    // /// | Time |
    // /// | ---- |
    // /// | O(N) |
    // ///
    // /// # Examples
    // ///
    // /// ```
    // /// use multiset_btree::BTreeMultiSet;
    // ///
    // /// let mut set = vec![1, 2, 2, 3, 4, 5, 5, 5].into_iter().collect::<BTreeMultiSet<_>>();
    // /// set.retain(|&x| x % 2 == 0);
    // /// assert_eq!(set.len(), 3);
    // /// assert_eq!(
    // ///     set.into_iter().collect::<Vec<_>>(),
    // ///     [2, 2, 4]
    // /// );
    // /// ```
    // pub fn retain<F>(&mut self, mut f: F)
    // where
    //     T: Ord,
    //     F: FnMut(&T) -> bool,
    // {
    //     let mut len = 0;
    //     self.tree.retain(|v, count| {
    //         if f(v) {
    //             len += *count;
    //             true
    //         } else {
    //             false
    //         }
    //     });
    //     self.len = len;
    // }
}

struct MultiSetElm<T, C>((T, C));

impl<T: Debug> Debug for MultiSetElm<T, &usize> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (v, &count) = &self.0;
        match count {
            0 => write!(f, ""),
            1 => write!(f, "{:?}", v),
            _ => write!(f, "{:?}*{:?}", v, count),
        }
    }
}

impl<T: Debug> Debug for BTreeMultiSet<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set()
            .entries(self.tree.iter().map(MultiSetElm))
            .finish()
    }
}

impl<T: Ord> Default for BTreeMultiSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: PartialEq> PartialEq for BTreeMultiSet<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && self.tree == other.tree
    }
}

impl<T: Eq> Eq for BTreeMultiSet<T> {}

impl<T: Hash> Hash for BTreeMultiSet<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.tree.hash(state);
    }
}

impl<T: Ord> Extend<T> for BTreeMultiSet<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        iter.into_iter().for_each(|v| self.insert(v));
    }
}

impl<T: Ord + Clone> BitOr<&BTreeMultiSet<T>> for &BTreeMultiSet<T> {
    type Output = BTreeMultiSet<T>;

    /// Returns the union of `self` and `rhs` as a new `BTreeMultiSet<T>`.
    ///
    /// # Complexity
    ///
    /// N: `self.len()`,
    /// M: `other.len()`
    ///
    /// | Time     |
    /// | -------- |
    /// | Θ(N + M) |
    ///
    /// # Examples
    ///
    /// ```
    /// use multiset_btree::BTreeMultiSet;
    ///
    /// let a = vec![1, 2, 2, 3].into_iter().collect::<BTreeMultiSet<_>>();
    /// let b = vec![2, 3, 4, 4].into_iter().collect::<BTreeMultiSet<_>>();
    ///
    /// let result = &a | &b;
    /// let result = result.into_iter().collect::<Vec<_>>();
    /// assert_eq!(result, [1, 2, 2, 2, 3, 3, 4, 4]);
    /// ```
    fn bitor(self, rhs: &BTreeMultiSet<T>) -> BTreeMultiSet<T> {
        self.union(rhs).cloned().collect()
    }
}

impl<T: Ord + Clone> BitAnd<&BTreeMultiSet<T>> for &BTreeMultiSet<T> {
    type Output = BTreeMultiSet<T>;

    /// Returns the intersection of `self` and `rhs` as a new `BTreeMultiSet<T>`.
    ///
    /// # Complexity
    ///
    /// N: `self.len()`,
    /// M: `other.len()`
    ///
    /// | Time     |
    /// | -------- |
    /// | O(N + M) |
    ///
    /// # Examples
    ///
    /// ```
    /// use multiset_btree::BTreeMultiSet;
    ///
    /// let a = vec![1, 2, 2, 3, 4].into_iter().collect::<BTreeMultiSet<_>>();
    /// let b = vec![2, 2, 2, 3, 3].into_iter().collect::<BTreeMultiSet<_>>();
    ///
    /// let result = &a & &b;
    /// let result = result.into_iter().collect::<Vec<_>>();
    /// assert_eq!(result, [2, 2, 3]);
    /// ```
    fn bitand(self, rhs: &BTreeMultiSet<T>) -> Self::Output {
        self.intersection(rhs).cloned().collect()
    }
}

impl<T: Ord + Clone> Sub<&BTreeMultiSet<T>> for &BTreeMultiSet<T> {
    type Output = BTreeMultiSet<T>;

    /// Returns the all elements of `self` that are not in `rhs` as a new `BTreeMultiSet<T>`.
    /// This is equivalent to collect [`difference`] iterator values.
    ///
    /// [`difference`]: BTreeMultiSet::difference
    ///
    /// # Complexity
    ///
    /// N: `self.len()`,
    /// M: `other.len()`
    ///
    /// | Time     |
    /// | -------- |
    /// | O(N + M) |
    ///
    /// # Examples
    ///
    /// ```
    /// use multiset_btree::BTreeMultiSet;
    ///
    /// let a = vec![1, 2, 2, 3, 4].into_iter().collect::<BTreeMultiSet<_>>();
    /// let b = vec![2, 3, 3].into_iter().collect::<BTreeMultiSet<_>>();
    ///
    /// let result = &a - &b;
    /// let result = result.into_iter().collect::<Vec<_>>();
    /// assert_eq!(result, [1, 2, 4]);
    /// ```
    fn sub(self, rhs: &BTreeMultiSet<T>) -> Self::Output {
        self.difference(rhs).cloned().collect()
    }
}

impl<T: Ord + Clone> BitXor<&BTreeMultiSet<T>> for &BTreeMultiSet<T> {
    type Output = BTreeMultiSet<T>;

    /// Returns the symmetric difference of `self` and `rhs` as a new `BTreeMultiSet<T>`.
    ///
    /// # Complexity
    ///
    /// N: `self.len()`,
    /// M: `other.len()`
    ///
    /// | Time     |
    /// | -------- |
    /// | O(N + M) |
    ///
    /// # Examples
    ///
    /// ```
    /// use multiset_btree::BTreeMultiSet;
    ///
    /// let a = vec![1, 2, 2, 3].into_iter().collect::<BTreeMultiSet<_>>();
    /// let b = vec![2, 3, 4, 4].into_iter().collect::<BTreeMultiSet<_>>();
    ///
    /// let result = &a ^ &b;
    /// let result = result.into_iter().collect::<Vec<_>>();
    /// assert_eq!(result, [1, 2, 4, 4]);
    /// ```
    fn bitxor(self, rhs: &BTreeMultiSet<T>) -> BTreeMultiSet<T> {
        self.symmetric_difference(rhs).cloned().collect()
    }
}

impl<T> From<BTreeMap<T, usize>> for BTreeMultiSet<T> {
    fn from(map: BTreeMap<T, usize>) -> Self {
        let len = map.iter().map(|(_, count)| count).sum();
        Self { tree: map, len }
    }
}

impl<T: Ord> FromIterator<T> for BTreeMultiSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = BTreeMultiSet::new();
        iter.into_iter().for_each(|value| {
            set.insert(value);
        });
        set
    }
}

impl<T> IntoIterator for BTreeMultiSet<T>
where
    T: Clone,
{
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

impl<'a, T> IntoIterator for &'a BTreeMultiSet<T>
where
    T: Clone + Ord,
{
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn from_iterator_vec() {
        let v = vec![1, 2, 3, 2, 2, 3, 4];
        let set = BTreeMultiSet::from_iter(v);
        assert_eq!(set.len(), 7);
        assert_eq!(set.into_iter().collect::<Vec<_>>(), [1, 2, 2, 2, 3, 3, 4]);
    }

    #[test]
    fn debug_format() {
        let set = vec![1, 2, 2, 3].into_iter().collect::<BTreeMultiSet<_>>();
        assert_eq!(format!("{:?}", set), "{1, 2*2, 3}");
    }

    #[test]
    fn eq() {
        let a = vec![1, 2, 2, 3].into_iter().collect::<BTreeMultiSet<_>>();
        let b = vec![1, 2, 2, 3].into_iter().collect::<BTreeMultiSet<_>>();
        assert!(a == b);
        assert!(b == a);

        let c = vec![1, 2, 3].into_iter().collect::<BTreeMultiSet<_>>();
        assert!(a != c);
        assert!(c != a);

        let d = vec![1, 2, 2, 2, 3, 4]
            .into_iter()
            .collect::<BTreeMultiSet<_>>();
        assert!(a != d);
        assert!(d != a);
    }

    #[test]
    fn hash() {
        let a = vec![1, 2, 2, 3].into_iter().collect::<BTreeMultiSet<_>>();
        let mut b = BTreeMultiSet::new();

        let mut hash_set = HashSet::new();
        hash_set.insert(a);

        assert!(!hash_set.contains(&b));
        b.insert(1);
        assert!(!hash_set.contains(&b));
        b.insert(2);
        assert!(!hash_set.contains(&b));
        b.insert(2);
        assert!(!hash_set.contains(&b));
        b.insert(3);
        assert!(hash_set.contains(&b));
        b.insert(2);
        assert!(!hash_set.contains(&b));
    }

    #[test]
    fn extend() {
        let mut a = vec![1, 2, 2, 3].into_iter().collect::<BTreeMultiSet<_>>();
        let b = vec![2, 3, 3, 4].into_iter().collect::<BTreeMultiSet<_>>();
        a.extend(b);
        assert_eq!(a.len(), 8);
        assert_eq!(a.into_iter().collect::<Vec<_>>(), [1, 2, 2, 2, 3, 3, 3, 4]);
    }
}
