use std::collections::VecDeque;

/// A queue with its elements in ascending order (unstrict)
#[derive(Default)]
pub struct MonotonicQueue<'a, T: PartialOrd> {
    inner: VecDeque<&'a T>,
}

impl<'a, T: PartialOrd> MonotonicQueue<'a, T> {
    /// Creates an empty MonotonicQueue.
    ///
    /// # Examples
    ///
    /// ```
    /// use rsalgo::ds::MonotonicQueue;
    ///
    /// let vector: MonotonicQueue<u32> = MonotonicQueue::new();
    /// ```
    pub fn new() -> Self {
        MonotonicQueue {
            inner: VecDeque::new(),
        }
    }

    /// Returns the number of elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use rsalgo::ds::MonotonicQueue;
    ///
    /// let v: MonotonicQueue<u32> = MonotonicQueue::new();
    /// assert_eq!(v.len(), 0);
    /// ```
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns true if the MonotonicQueue is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use rsalgo::ds::MonotonicQueue;
    ///
    /// let v: MonotonicQueue<u32> = MonotonicQueue::new();
    /// assert!(v.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Clears the MonotonicQueue, removing all values.
    ///
    /// # Examples
    ///
    /// ```
    /// use rsalgo::ds::MonotonicQueue;
    ///
    /// let mut v: MonotonicQueue<u32> = MonotonicQueue::new();
    /// v.enqueue(&1);
    /// v.clear();
    /// assert!(v.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.inner.clear()
    }

    /// Appends an element to the back of the MonotonicQueue.
    ///
    /// # Examples
    ///
    /// ```
    /// use rsalgo::ds::MonotonicQueue;;
    ///
    /// let mut buf = MonotonicQueue::new();
    /// buf.enqueue(&1);
    /// buf.enqueue(&3); // removed when enqueue `2`
    /// buf.enqueue(&2);
    /// assert_eq!(2, buf.len());
    /// ```
    pub fn enqueue(&mut self, value: &'a T) {
        while let Some(back) = self.inner.back() {
            if back <= &value {
                break;
            }
            self.inner.pop_back();
        }
        self.inner.push_back(value);
    }

    /// Removes the first element and returns it, or None if the MonotonicQueue is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use rsalgo::ds::MonotonicQueue;
    /// let mut d = MonotonicQueue::new();
    /// d.enqueue(&1);
    /// d.enqueue(&2);
    ///
    /// assert_eq!(d.dequeue(), Some(&1));
    /// assert_eq!(d.dequeue(), Some(&2));
    /// assert_eq!(d.dequeue(), None);
    /// ```
    pub fn dequeue(&mut self) -> Option<&T> {
        self.inner.pop_front()
    }

    /// Provides a reference to the front element, or None if the MonotonicQueue is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use rsalgo::ds::MonotonicQueue;
    /// let mut d = MonotonicQueue::new();
    /// d.enqueue(&1);
    /// d.enqueue(&2);
    ///
    /// assert_eq!(d.head(), Some(&&1));
    /// ```
    pub fn head(&self) -> Option<&&T> {
        self.inner.front()
    }
}

#[cfg(test)]
mod tests {
    use super::MonotonicQueue;

    #[test]
    fn monotonic_queue() {
        let a = [1, 2, 2, 1, 3, 9, 4, 5, 20, 80];
        let expect = [1, 1, 3, 4, 5, 20, 80];

        let mut q: MonotonicQueue<u32> = MonotonicQueue::new();
        assert!(q.is_empty());

        for x in &a {
            q.enqueue(x);
        }

        assert_eq!(q.len(), expect.len());

        for x in &expect {
            assert_eq!(&x, q.head().unwrap());
            assert_eq!(x, q.dequeue().unwrap());
        }

        assert!(q.is_empty());
    }
}
