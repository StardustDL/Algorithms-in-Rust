use super::search;

/// Get the number of inversion pairs.
///
/// # Examples
///
/// ```
/// let slice = [1, 3, 5, 3, 2, 4, 7, 9, 6, 6];
/// assert_eq!(9, rsalgo::base::inversion_pairs_count(&slice));
/// ```
pub fn inversion_pairs_count<T: PartialEq + Ord + Clone>(slice: &[T]) -> usize {
    let mut ans = 0;
    let mut temp = vec![0; slice.len()];
    merge_sort(
        search::discretization(slice).as_mut_slice(),
        0,
        slice.len(),
        &mut temp,
        &mut ans,
    );
    ans
}

fn merge_sort(a: &mut [usize], l: usize, r: usize, temp: &mut [usize], pair: &mut usize) {
    if l + 1 >= r {
        return;
    }
    let mid = (l + r) / 2;
    merge_sort(a, l, mid, temp, pair);
    merge_sort(a, mid, r, temp, pair);
    let (mut ll, mut rr, mut cur) = (l, mid, 0);

    while ll < mid && rr < r {
        if a[ll] <= a[rr] {
            temp[cur] = a[ll];
            ll += 1;
        } else {
            *pair += mid - ll;
            temp[cur] = a[rr];
            rr += 1;
        }
        cur += 1;
    }

    while ll < mid {
        temp[cur] = a[ll];
        cur += 1;
        ll += 1;
    }

    while rr < r {
        temp[cur] = a[rr];
        cur += 1;
        rr += 1;
    }

    a[l..r].clone_from_slice(&temp[0..(r - l)]);
}

#[cfg(test)]
mod tests {
    #[test]
    fn inversion_pairs_count() {
        assert_eq!(
            9,
            super::inversion_pairs_count(&[1, 3, 5, 3, 2, 4, 7, 9, 6, 6])
        );
    }
}
