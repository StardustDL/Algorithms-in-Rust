/// Disjoint set
pub struct DisjointSet {
    size: usize,
    parent: Vec<usize>,
    rank: Option<Vec<usize>>,
}

impl DisjointSet {
    /// Create a disjoint set with `size` elements. Union by rank if `enable_rank` is true.
    ///
    /// # Examples
    ///
    /// ```
    /// let s = rsalgo::ds::DisjointSet::new(100, true);
    /// ```
    pub fn new(size: usize, enable_rank: bool) -> Self {
        DisjointSet {
            size: size,
            parent: (0..size).collect(),
            rank: if enable_rank {
                Some(vec![0; size])
            } else {
                None
            },
        }
    }

    /// Get the number of elements. This will use path compression.
    ///
    /// # Examples
    ///
    /// ```
    /// let s = rsalgo::ds::DisjointSet::new(100, true);
    ///
    /// assert_eq!(100, s.size());
    /// ```
    pub fn size(&self) -> usize {
        self.size
    }

    /// Get the representation element of `id`'s set.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut s = rsalgo::ds::DisjointSet::new(100, true);
    ///
    /// assert_eq!(0, s.find(0));
    /// ```
    pub fn find(&mut self, id: usize) -> usize {
        if self.parent[id] == id {
            id
        } else {
            self.parent[id] = self.find(self.parent[id]);
            self.parent[id]
        }
    }

    /// Return whether the two element is in the same set.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut s = rsalgo::ds::DisjointSet::new(100, true);
    ///
    /// assert_eq!(false, s.issame(0, 1));
    /// ```
    pub fn issame(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    /// Unite `a`'s set and `b`'s set.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut s = rsalgo::ds::DisjointSet::new(100, true);
    /// s.unite(0, 1);
    /// assert_eq!(true, s.issame(0, 1));
    /// ```
    pub fn unite(&mut self, a: usize, b: usize) {
        let (mut a, mut b) = (self.find(a), self.find(b));
        if a == b {
            return;
        }
        if let Some(rk) = &mut self.rank {
            if rk[a] < rk[b] {
                std::mem::swap(&mut a, &mut b);
            }
            rk[a] += if rk[a] == rk[b] { 1 } else { 0 }
        }
        self.parent[b] = a;
    }
}

#[cfg(test)]
mod tests {
    use super::DisjointSet;
    use rand::Rng;

    #[test]
    fn disjoint_set() {
        let mut rng = rand::thread_rng();

        const SIZE: usize = 100;

        let mut sa = DisjointSet::new(SIZE, true);
        let mut sb = DisjointSet::new(SIZE, false);

        for _ in 0..5 {
            let a = rng.gen_range(0, SIZE);
            let b = rng.gen_range(0, SIZE);

            assert_eq!(sa.issame(a, b), sb.issame(a, b));

            sa.unite(a, b);
            sb.unite(a, b);

            assert!(sa.issame(a, b));
            assert!(sb.issame(a, b));
        }
    }
}
