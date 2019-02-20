use super::Uint;

/// Test if `value` is prime by trial division `O(sqrt(value))`.
///
/// # Examples
///
/// ```
/// assert!(rsalgo::math::number_theory::is_prime_trial(2));
/// assert!(!rsalgo::math::number_theory::is_prime_trial(8));
/// ```
///
/// # Panics
///
/// Panic when `value` equals to `0` or `1`.
pub fn is_prime_trial(value: Uint) -> bool {
    if value == 0 || value == 1 {
        panic!("0 and 1 are not accepted.");
    }

    let sqrt = (value as f64).sqrt().floor() as Uint;

    (2..sqrt + 1).all(|x| value % x != 0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn trial() {
        assert!(super::is_prime_trial(2));
        assert!(super::is_prime_trial(3));
        assert!(!super::is_prime_trial(4));
        assert!(super::is_prime_trial(5));
        assert!(!super::is_prime_trial(6));
        assert!(super::is_prime_trial(7));
    }
}
