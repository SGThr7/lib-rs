/// `UnionFind` can process mainly two queries.
///
/// - **Union**: merge two sets.
/// - **Find**: find a representative member of a set.
pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    /// Create `n` disjoint-sets.
    pub fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        let size = vec![1; n];
        UnionFind { parent, size }
    }

    /// Returns the number of elements in the struct.
    pub fn len(&self) -> usize {
        self.parent.len()
    }

    /// Returns `true` if the sets has a length of 0.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns `true` if `a` and `b` are in the same set.
    ///
    /// # Examples
    ///
    /// ```
    /// # use unionfind::UnionFind;
    /// let mut uf = UnionFind::new(3);
    ///
    /// assert!( uf.equiv(0, 0));
    /// assert!(!uf.equiv(0, 1));
    /// assert!(!uf.equiv(0, 2));
    ///
    /// uf.union(0, 2);
    /// assert!(!uf.equiv(0, 1));
    /// assert!( uf.equiv(0, 2));
    /// assert!( uf.equiv(2, 0));
    /// ```
    pub fn equiv(&self, a: usize, b: usize) -> bool {
        self.find_root(a) == self.find_root(b)
    }

    /// Merge the set containing `a` with the set containing `b`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use unionfind::UnionFind;
    /// let mut uf = UnionFind::new(3);
    ///
    /// uf.union(0, 2);
    ///
    /// assert_eq!(uf.find_root(0), uf.find_root(2));
    /// ```
    pub fn union(&mut self, a: usize, b: usize) {
        if a == b {
            // same node
            return;
        }

        let a_root = self.find_root(a);
        let b_root = self.find_root(b);
        if a_root == b_root {
            // same parent
            return;
        }

        let a_size = self.size[a_root];
        let b_size = self.size[b_root];
        let f = |(a, _a_size), (b, _b_size)| a <= b;
        if f((a_root, a_size), (b_root, b_size)) {
            self.parent[b_root] = a_root;
            self.size[a_root] += b_size;
        } else {
            self.parent[a_root] = b_root;
            self.size[b_root] += a_size;
        }
    }

    /// Find a representative member of a set including `x`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use unionfind::UnionFind;
    /// let mut uf = UnionFind::new(3);
    ///
    /// assert_ne!(uf.find_root(0), uf.find_root(1));
    ///
    /// uf.union(0, 1);
    /// assert_eq!(uf.find_root(0), uf.find_root(1));
    /// ```
    pub fn find_root(&self, mut x: usize) -> usize {
        assert!(x < self.len());
        while x != self.parent[x] {
            // while not root
            x = self.parent[x];
        }
        x
    }

    /// Find a representative member of a set including `x` with caching.
    pub fn find_root_caching(&mut self, x: usize) -> usize {
        assert!(x < self.len());
        let p = self.parent[x];
        if x == p {
            // x is root
            x
        } else {
            let root = self.find_root_caching(p);
            self.parent[x] = root;
            root
        }
    }

    /// Get the size of the set including `x`.
    ///
    /// # Examples
    /// ```
    /// # use unionfind::UnionFind;
    /// let mut uf = UnionFind::new(5);
    ///
    /// assert_eq!(uf.get_size(2), 1);
    ///
    /// uf.union(0, 2);
    /// assert_eq!(uf.get_size(2), 2);
    ///
    /// uf.union(1, 4);
    /// uf.union(0, 4);
    /// assert_eq!(uf.get_size(2), 4);
    /// ```
    pub fn get_size(&self, x: usize) -> usize {
        self.size[self.find_root(x)]
    }

    /// Get the size of the set including `x`.
    pub fn get_size_caching(&mut self, x: usize) -> usize {
        let root = self.find_root_caching(x);
        self.size[root]
    }
}
