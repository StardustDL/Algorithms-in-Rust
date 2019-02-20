use std::ops::{Add, Range, Sub};

/// 1D prefix sum
pub struct PrefixSum1D<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T>,
{
    inner: Vec<<T as Add>::Output>,
}

impl<T> PrefixSum1D<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T>,
{
    /// Creates a PrefixSum1D from `values`
    /// 
    /// # Examples
    /// 
    /// ```
    /// let source: Vec<usize> = (0..100).collect();
    /// let ps = PrefixSum1D::new(&source, 0);
    /// 
    /// assert_eq!(ps.sum(0..100), (0..100).sum());
    /// ```
    pub fn new(values: &[T], zero: T) -> Self {
        let mut inner = Vec::with_capacity(values.len() + 1);
        inner.push(zero);
        for value in values {
            inner.push(inner[inner.len() - 1] + *value);
        }
        PrefixSum1D { inner }
    }

    /// Gets sum of the `range` in basic sequence.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let source: Vec<usize> = (0..100).collect();
    /// let ps = PrefixSum1D::new(&source, 0);
    /// 
    /// assert_eq!(ps.sum(1..1), (1..1).sum());
    /// assert_eq!(ps.sum(10..15), (10..15).sum());
    /// ```
    pub fn sum(&self, range: Range<usize>) -> T {
        // use .is_empty when it stable
        self.inner[range.end] - self.inner[range.start]
    }
}

#[cfg(test)]
mod tests {
    use super::PrefixSum1D;

    #[test]
    fn d1() {
        let source: Vec<usize> = (0..100).collect();
        let ps = PrefixSum1D::new(&source, 0);

        assert_eq!(ps.sum(1..1), (1..1).sum());
        assert_eq!(ps.sum(10..15), (10..15).sum());
        assert_eq!(ps.sum(0..100), (0..100).sum());
    }
}
