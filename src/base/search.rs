/// Dichotomy in interval `[lower_bound, upper_bound)`, return the first value that makes checker true.
///
/// # Examples
///
/// ```
/// assert_eq!(Some(5), rsalgo::base::dichotomy(0, 10, |val| val >= 5));
/// ```
pub fn dichotomy<TC: Fn(isize) -> bool>(
    lower_bound: isize,
    upper_bound: isize,
    checker: TC,
) -> Option<isize> {
    let mut l = lower_bound;
    let mut r = upper_bound;
    let mut ans = None;

    while l < r {
        let mid = (l + r) / 2;
        if checker(mid) {
            ans = Some(mid);
            r = mid;
        } else {
            l = mid + 1;
        }
    }

    ans
}

/// Get the position of the lower bound of value in the slice
///
/// # Examples
///
/// ```
/// let slice = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
///
/// assert_eq!(Some(5), rsalgo::base::lower_bound(&slice, &5));
/// ```
pub fn lower_bound<T: PartialOrd>(slice: &[T], value: &T) -> Option<usize> {
    match dichotomy(0, slice.len() as isize, |pos| &slice[pos as usize] >= value) {
        Some(val) => Some(val as usize),
        None => None,
    }
}

/// Get the position of the upper bound of value in the slice
///
/// # Examples
///
/// ```
/// let slice = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
///
/// assert_eq!(Some(6), rsalgo::base::upper_bound(&slice, &5));
/// ```
pub fn upper_bound<T: PartialOrd>(slice: &[T], value: &T) -> Option<usize> {
    match dichotomy(0, slice.len() as isize, |pos| &slice[pos as usize] > value) {
        Some(val) => Some(val as usize),
        None => None,
    }
}

/// Get the position interval that those values equal to the value
///
/// # Examples
///
/// ```
/// let slice = [0, 1, 2, 2, 2, 5, 6, 7, 8, 9];
///
/// assert_eq!(Some((2, 5)), rsalgo::base::equal_range(&slice, &2));
/// ```
pub fn equal_range<T: PartialOrd>(slice: &[T], value: &T) -> Option<(usize, usize)> {
    match (lower_bound(slice, value), upper_bound(slice, value)) {
        (Some(first), Some(last)) => Some((first, last)),
        (Some(first), None) => Some((first, slice.len() as usize)),
        (None, Some(last)) => Some((0, last)),
        (None, None) => None,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn dichotomy() {
        assert_eq!(Some(5), super::dichotomy(0, 10, |val| val >= 5));
        assert_eq!(None, super::dichotomy(0, 10, |val| val > 10));
    }

    #[test]
    fn lower_bound() {
        let slice = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(Some(5), super::lower_bound(&slice, &5));
        assert_eq!(Some(0), super::lower_bound(&slice, &-1));
        assert_eq!(None, super::lower_bound(&slice, &10));
    }

    #[test]
    fn upper_bound() {
        let slice = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(Some(6), super::upper_bound(&slice, &5));
        assert_eq!(Some(0), super::upper_bound(&slice, &-1));
        assert_eq!(None, super::upper_bound(&slice, &10));
    }

    #[test]
    fn equal_range() {
        let slice = [0, 1, 2, 2, 2, 5, 6, 7, 8, 9];
        assert_eq!(Some((2, 5)), super::equal_range(&slice, &2));
    }
}
