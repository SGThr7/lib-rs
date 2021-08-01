#[codesnip::entry("BIT")]
pub struct BinaryIndexedTree<T> {
    pub(crate) tree: Vec<T>,
}

#[codesnip::entry("BIT")]
impl<T> BinaryIndexedTree<T>
where
    T: Copy + num::Zero,
{
    pub fn new(size: usize) -> Self {
        Self {
            tree: vec![T::zero(); size],
        }
    }

    pub fn len(&self) -> usize {
        self.tree.len()
    }

    pub fn add(&mut self, i: usize, x: T) {
        let mut i = i as isize;
        while (i as usize) < self.tree.len() {
            self.tree[i as usize] = self.tree[i as usize] + x;
            i += (i + 1) & -(i + 1);
        }
    }

    pub fn sum(&self, i: usize) -> T {
        let mut res = T::zero();
        let mut i = i as isize;
        while i >= 0 {
            res = res + self.tree[i as usize];
            i -= (i + 1) & -(i + 1);
        }
        res
    }
}

#[codesnip::entry("BIT")]
impl<T> From<Vec<T>> for BinaryIndexedTree<T>
where
    T: Copy + num::Zero,
{
    fn from(ar: Vec<T>) -> Self {
        let mut bit = BinaryIndexedTree::new(ar.len());
        for i in 0..ar.len() {
            bit.add(i, ar[i]);
        }
        bit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        const LEN: usize = 10;
        let bit = BinaryIndexedTree::<i32>::new(LEN);
        assert_eq!(bit.tree, [0; LEN]);
        assert_eq!(bit.len(), LEN)
    }

    #[test]
    fn from() {
        let bit = BinaryIndexedTree::from(vec![1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(bit.tree, [1, 3, 3, 10, 5, 11, 7, 36]);
        assert_eq!(bit.len(), 8);
    }

    #[test]
    fn add() {
        let mut bit = BinaryIndexedTree::new(8);
        bit.add(0, 3);
        assert_eq!(bit.tree, [3, 3, 0, 3, 0, 0, 0, 3]);
    }

    #[test]
    fn sum() {
        let bit = BinaryIndexedTree::from(vec![1, 2, 3, 4, 5, 6, 7, 8]);
        let sum = {
            let mut res = Vec::with_capacity(bit.len());
            for i in 0..bit.len() {
                res.push(bit.sum(i));
            }
            res
        };
        assert_eq!(sum, [1, 3, 6, 10, 15, 21, 28, 36]);
    }
}
