//! Number theory related algorithms

/// Get the greatest common divisor between `a` and `b`
///
/// # Examples
///
/// ```
/// assert_eq!(12, rsalgo::math::number_theory::gcd(24, 36));
/// ```
pub fn gcd(a: usize, b: usize) -> usize {
    let (mut a, mut b) = (a, b);
    while b > 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

/// Get the least common multiple between `a` and `b`
///
/// # Examples
///
/// ```
/// assert_eq!(72, rsalgo::math::number_theory::lcm(24, 36));
/// ```
pub fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

/// Get the value of `base^exp % modulo`
///
/// # Examples
///
/// ```
/// assert_eq!(25, rsalgo::math::number_theory::quick_pow(5, 7, 100));
/// ```
pub fn quick_pow(base: usize, exp: usize, modulo: usize) -> usize {
    let mut ans = 1;
    let (mut base, mut exp) = (base % modulo, exp);
    while exp > 0 {
        if exp % 2 != 0 {
            ans *= base;
            ans %= modulo;
        }
        base *= base;
        base %= modulo;
        exp /= 2;
    }
    ans
}

/// Get the value of `base*exp % modulo`
///
/// # Examples
///
/// ```
/// assert_eq!(5, rsalgo::math::number_theory::quick_multiply(5, 7, 10));
/// ```
pub fn quick_multiply(a: usize, b: usize, modulo: usize) -> usize {
    let mut ans = 0;
    let (mut a, mut b) = (a % modulo, b % modulo);
    while b > 0 {
        if b % 2 != 0 {
            ans += a;
            ans %= modulo;
        }
        a += a;
        a %= modulo;
        b /= 2;
    }
    ans
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    #[test]
    fn gcd() {
        assert_eq!(3, super::gcd(3, 6));
        assert_eq!(12, super::gcd(24, 36));
        assert_eq!(5, super::gcd(5, 0));
    }

    #[test]
    fn lcm() {
        assert_eq!(6, super::lcm(3, 6));
        assert_eq!(72, super::lcm(24, 36));
        assert_eq!(245, super::lcm(35, 49));
        assert_eq!(0, super::lcm(5, 0));
    }

    #[test]
    fn quick_pow_multiply_test() {
        let mut rng = rand::thread_rng();

        for _ in 0..5 {
            let a = rng.gen_range(0, 1000);
            let b = rng.gen_range(0, 1000);
            let md = rng.gen_range(1, 1000);

            assert_eq!(a * b % md, super::quick_multiply(a, b, md));

            let mut rpow = 1;
            for _ in 0..b {
                rpow *= a;
                rpow %= md;
            }

            assert_eq!(rpow, super::quick_pow(a, b, md));
        }
    }
}
