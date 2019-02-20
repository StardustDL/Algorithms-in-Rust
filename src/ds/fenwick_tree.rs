use std::ops::{Add, Sub};

// Fenwick tree for point added and prefix sum
pub struct FenwickTree<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T>,
{
    inner: Vec<<T as Add>::Output>,
    zero: T,
}

impl<T> FenwickTree<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T>,
{
    /// Creates an empty FenwickTree with `zero`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rsalgo::ds::FenwickTree;
    ///
    /// let ft: FenwickTree<u32> = FenwickTree::new(10, 0);
    /// ```
    pub fn new(size: usize, zero: T) -> Self {
        FenwickTree {
            inner: vec![zero; size],
            zero,
        }
    }

    /// Returns the length.
    ///
    /// # Examples
    ///
    /// ```
    /// use rsalgo::ds::FenwickTree;
    ///
    /// let v: FenwickTree<u32> = FenwickTree::new(10, 0);
    /// assert_eq!(v.len(), 10);
    /// ```
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns if FenwickTree is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use rsalgo::ds::FenwickTree;
    ///
    /// let v: FenwickTree<u32> = FenwickTree::new(0, 0);
    /// assert!(v.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Add `delta` to the position `pos`
    ///
    /// # Examples
    ///
    /// ```
    /// use rsalgo::ds::FenwickTree;
    ///
    /// let mut v: FenwickTree<u32> = FenwickTree::new(10, 0);
    /// v.add(0, 1);
    /// assert_eq!(1, v.prefix_sum(1));
    /// ```
    pub fn add(&mut self, pos: usize, delta: T) {
        let mut pos = pos + 1;
        while pos <= self.len() {
            self.inner[pos - 1] = self.inner[pos - 1] + delta;
            pos += 1 << pos.trailing_zeros();
        }
    }

    /// Gets sum of `[0, end)` in basic sequence.
    ///
    /// # Examples
    ///
    /// ```
    /// use rsalgo::ds::FenwickTree;
    /// let mut v: FenwickTree<u32> = FenwickTree::new(10, 0);
    /// v.add(0, 1);
    /// v.add(1, 2);
    /// assert_eq!(3, v.prefix_sum(2));
    /// ```
    pub fn prefix_sum(&self, end: usize) -> T {
        let mut res = self.zero;
        let mut end = end;
        while end != 0 {
            res = res + self.inner[end - 1];
            end -= 1 << end.trailing_zeros();
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::FenwickTree;
    use rand::Rng;

    #[test]
    fn m1_qp() {
        const LEN: usize = 100;
        let mut source: Vec<usize> = vec![0; LEN];
        let mut ft = FenwickTree::new(LEN, 0);

        let mut rng = rand::thread_rng();

        for _ in 1..1000 {
            if rng.gen_ratio(2, 3) {
                let pos = rng.gen_range(0, LEN);
                let delta = rng.gen_range(0, LEN);
                source[pos] += delta;
                ft.add(pos, delta);
            } else {
                let pos = rng.gen_range(0, LEN);
                let mut exp = 0;
                for i in 0..pos {
                    exp += source[i]
                }
                assert_eq!(exp, ft.prefix_sum(pos));
            }
        }
    }
}
