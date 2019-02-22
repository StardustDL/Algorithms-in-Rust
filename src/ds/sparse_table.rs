use std::ops::Range;

pub struct SparseTable<'a, T: Ord> {
    inner: Vec<Vec<&'a T>>,
}

impl<'a, T: Ord> SparseTable<'a, T> {
    pub fn new(values: &'a [T]) -> Self {
        let n = values.len();
        let pow2 = n.next_power_of_two().trailing_zeros() as usize;
        let mut inner = vec![vec![]; n];

        inner
            .iter_mut()
            .zip(values)
            .for_each(|(inn, v)| inn.push(v));

        for j in 1..=pow2 {
            for i in 0..=(n as isize) - (1 << j) {
                let left = inner[i as usize][j - 1];
                let right = inner[i as usize + (1 << (j - 1))][j - 1];
                inner[i as usize].push(std::cmp::min(left, right));
            }
        }

        SparseTable { inner }
    }

    pub fn min(&self, range: Range<usize>) -> &'a T {
        // TODO
        let len = range.end - range.start;
        if len.is_power_of_two() {
            self.inner[range.start][len.trailing_zeros() as usize]
        } else {
            let k = ((len as f64).log2().floor()) as usize; // Error
            std::cmp::min(
                self.inner[range.start][k],
                self.inner[range.end - (1 << k)][k],
            )
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::SparseTable;
    use rand::Rng;

    #[test]
    fn st() {
        const LEN: usize = 50;
        let mut rng = rand::thread_rng();
        let mut ori = [0; LEN];
        rng.fill(&mut ori[..]);
        let st = SparseTable::new(&ori[..]);

        for _ in 0..1000 {
            let l = rng.gen_range(0, LEN / 2);
            let r = rng.gen_range(LEN / 2, LEN);
            dbg!((l, r));
            assert_eq!(ori[l..r].iter().min().unwrap(), st.min(l..r));
        }
    }
}
