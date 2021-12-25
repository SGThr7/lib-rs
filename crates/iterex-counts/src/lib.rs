use std::{collections::HashMap, hash::Hash};

pub trait IterExCounts: Iterator {
    /// Counts the number of items in this iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// use sgthr7_lib_iterex_counts::IterExCounts;
    ///
    /// let counts = [1, 1, 1, 3, 3, 5].into_iter().counts();
    ///
    /// assert_eq!(counts[&1], 3);
    /// assert_eq!(counts[&3], 2);
    /// assert_eq!(counts[&5], 1);
    /// assert_eq!(counts.get(&0), None);
    /// ```
    fn counts(self) -> HashMap<Self::Item, usize>
    where
        Self: Sized,
        Self::Item: Eq + Hash,
    {
        self.fold(HashMap::new(), |mut map, item| {
            *map.entry(item).or_default() += 1;
            map
        })
    }
}

impl<I: Iterator> IterExCounts for I {}
