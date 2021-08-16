#[codesnip::entry]
pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

#[codesnip::entry("UnionFind")]
impl UnionFind {
    pub fn new(n: usize) -> Self {
        let parent = (0..n).collect::<Vec<_>>();
        let size = vec![1; n];
        Self { parent, size }
    }

    pub fn len(&self) -> usize {
        self.parent.len()
    }

    pub fn find(&self, x: usize) -> usize {
        assert!(x < self.parent.len());
        let parent = self.parent[x];
        if parent != x {
            self.find(parent)
        } else {
            parent
        }
    }

    pub fn find_mut(&mut self, x: usize) -> usize {
        assert!(x < self.parent.len());
        let parent = self.parent[x];
        if parent != x {
            let root = self.find_mut(parent);
            self.parent[x] = root;
            root
        } else {
            parent
        }
    }

    pub fn equiv(&self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    pub fn size(&self, x: usize) -> usize {
        self.size[self.find(x)]
    }

    pub fn size_mut(&mut self, x: usize) -> usize {
        let root = self.find_mut(x);
        self.size[root]
    }

    pub fn union(&mut self, a: usize, b: usize) -> bool {
        if a == b {
            return false;
        }

        let a = self.find_mut(a);
        let b = self.find_mut(b);
        if a == b {
            return false;
        }

        let asize = self.size[a];
        let bsize = self.size[b];
        if asize > bsize {
            self.parent[b] = a;
            self.size[a] += bsize;
        } else {
            self.parent[a] = b;
            self.size[b] += asize;
        }
        true
    }
}
