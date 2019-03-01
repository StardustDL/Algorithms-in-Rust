use super::Uint;

/// Test if `value` is prime by trial division O(sqrt(value)).
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

    (2..=sqrt).all(|x| value % x != 0)
}

// Miller rabin
pub fn is_prime(value: Uint) -> bool {
    fn witness(a: Uint, u: Uint, t: Uint, value: Uint) -> bool {
        let mut x0 = super::quick_pow(a, u, value);
        for _ in 0..t {
            let x1 = (x0 * x0) % value;
            if x1 == 1 && x0 != 1 && x0 != (value - 1) {
                return true;
            }
            x0 = x1;
        }
        x0 != 1
    }

    if value == 0 || value == 1 {
        panic!("0 and 1 are not accepted.");
    }

    use rand::Rng;
    const TEST: usize = 32;

    let mut rng = rand::thread_rng();
    let t = (value - 1).trailing_zeros() as Uint;
    let u = (value - 1) >> t;
    for _ in 0..TEST {
        let a = rng.gen_range(1, value);
        if witness(a, u, t, value) {
            return false;
        }
    }
    true
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
