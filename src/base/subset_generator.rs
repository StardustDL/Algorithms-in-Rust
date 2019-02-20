/// Subset sequence generator
pub struct SubsetGenerator {
    set: usize,
    full: usize,
}

impl SubsetGenerator {
    /// Create from the fullset and the current subset.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut gen = rsalgo::base::SubsetGenerator::new(0b100, 0b110);
    ///
    /// assert_eq!(Some(0b010), gen.next());
    /// ```
    pub fn new(subset: usize, fullset: usize) -> Self {
        SubsetGenerator {
            set: subset,
            full: fullset,
        }
    }
}

impl Iterator for SubsetGenerator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.set == 0 {
            return None;
        }
        self.set = (self.set - 1) & self.full;
        Some(self.set)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn next_subset() {
        let mut gen = super::SubsetGenerator::new(0b110, 0b110);
        assert_eq!(Some(0b100), gen.next());
        assert_eq!(Some(0b010), gen.next());
        assert_eq!(Some(0b000), gen.next());
        assert_eq!(None, gen.next());
    }
}
