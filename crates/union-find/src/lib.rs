pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        let size = vec![1; n];
        Self { parent, size }
    }

    pub fn len(&self) -> usize {
        self.parent.len()
    }

    pub fn equiv(&self, a: usize, b: usize) -> bool {
        self.find_root(a) == self.find_root(b)
    }

    pub fn union(&mut self, a: usize, b: usize) {
        self.union_by(a, b, |(a, _a_size), (b, _b_size)| a <= b)
    }

    pub fn union_by<F>(&mut self, a: usize, b: usize, mut union_to_left: F)
    where
        F: FnMut((usize, usize), (usize, usize)) -> bool,
    {
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
        if union_to_left((a_root, a_size), (b_root, b_size)) {
            self.parent[b_root] = a_root;
            self.size[a_root] += b_size;
        } else {
            self.parent[a_root] = b_root;
            self.size[b_root] += a_size;
        }
    }

    pub fn find_root(&self, mut x: usize) -> usize {
        assert!(x < self.len());
        while x != self.parent[x] {
            // while not root
            x = self.parent[x];
        }
        x
    }

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

    pub fn get_size(&self, x: usize) -> usize {
        self.size[self.find_root(x)]
    }

    pub fn get_size_caching(&mut self, x: usize) -> usize {
        let root = self.find_root_caching(x);
        self.size[root]
    }
}
