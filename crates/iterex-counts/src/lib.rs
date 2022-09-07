use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
};

pub trait IterExCounts: Iterator {
    /// Counts the number of items in this iterator with [`HashMap`].
    ///
    /// # Examples
    ///
    /// ```
    /// use iterex_counts::IterExCounts;
    /// use std::collections::HashMap;
    ///
    /// let counts: HashMap<_, usize> = [1, 1, 1, 3, 3, 5].into_iter().counts_map();
    ///
    /// assert_eq!(counts[&1], 3);
    /// assert_eq!(counts[&3], 2);
    /// assert_eq!(counts[&5], 1);
    /// assert_eq!(counts.get(&0), None);
    /// ```
    fn counts_map(self) -> HashMap<Self::Item, usize>
    where
        Self: Sized,
        Self::Item: Eq + Hash,
    {
        self.fold(HashMap::new(), |mut map, item| {
            *map.entry(item).or_default() += 1;
            map
        })
    }

    /// Counts the number of items in this iterator with [`BTreeMap`].
    ///
    /// # Examples
    ///
    /// ```
    /// use iterex_counts::IterExCounts;
    /// use std::collections::BTreeMap;
    ///
    /// let counts: BTreeMap<_, usize> = [1, 1, 1, 3, 3, 5].into_iter().counts_btree();
    ///
    /// assert_eq!(counts[&1], 3);
    /// assert_eq!(counts[&3], 2);
    /// assert_eq!(counts[&5], 1);
    /// assert_eq!(counts.get(&0), None);
    /// ```
    fn counts_btree(self) -> BTreeMap<Self::Item, usize>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        self.fold(BTreeMap::new(), |mut map, item| {
            *map.entry(item).or_default() += 1;
            map
        })
    }
}

impl<I: Iterator> IterExCounts for I {}
