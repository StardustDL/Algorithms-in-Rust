/// Disjoint set
pub struct DisjointSet {
    capacity: usize,
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
    pub fn new(capacity: usize, enable_rank: bool) -> Self {
        DisjointSet {
            capacity,
            size: capacity,
            parent: (0..capacity).collect(),
            rank: if enable_rank {
                Some(vec![0; capacity])
            } else {
                None
            },
        }
    }

    /// Get the number of disjoint sets.
    ///
    /// # Examples
    ///
    /// ```
    /// let s = rsalgo::ds::DisjointSet::new(100, true);
    ///
    /// assert_eq!(100, s.len());
    /// ```
    pub fn len(&self) -> usize {
        self.size
    }

    /// Get the number of elements.
    ///
    /// # Examples
    ///
    /// ```
    /// let s = rsalgo::ds::DisjointSet::new(100, true);
    ///
    /// assert_eq!(100, s.capacity());
    /// ```
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Returns true if the set contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// let s = rsalgo::ds::DisjointSet::new(0, true);
    ///
    /// assert!(s.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.capacity == 0
    }

    pub fn is_one(&self) -> bool {
        self.size == 1
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
    /// assert_eq!(false, s.in_same(0, 1));
    /// ```
    pub fn in_same(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    /// Unite `a`'s set and `b`'s set.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut s = rsalgo::ds::DisjointSet::new(100, true);
    /// s.unite(0, 1);
    /// assert_eq!(true, s.in_same(0, 1));
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
        self.size -= 1;
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

        assert_eq!(SIZE, sa.len());

        for _ in 0..5 {
            let a = rng.gen_range(0, SIZE);
            let b = rng.gen_range(0, SIZE);

            assert_eq!(sa.in_same(a, b), sb.in_same(a, b));

            sa.unite(a, b);
            sb.unite(a, b);

            assert!(sa.in_same(a, b));
            assert!(sb.in_same(a, b));
        }
    }
}
