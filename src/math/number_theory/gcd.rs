use super::Uint;

/// Get the greatest common divisor between `a` and `b` by Enclid Algorithm
///
/// # Examples
///
/// ```
/// assert_eq!(12, rsalgo::math::number_theory::gcd_euclid(24, 36));
/// ```
pub fn gcd_euclid(a: Uint, b: Uint) -> Uint {
    let (mut a, mut b) = (a, b);

    while b != 0 {
        std::mem::swap(&mut a, &mut b);
        b %= a;
    }

    a
}

/// Get the greatest common divisor between `a` and `b` by Stein's algorithm
///
/// # Examples
///
/// ```
/// assert_eq!(12, rsalgo::math::number_theory::gcd(24, 36));
/// ```
pub fn gcd(a: Uint, b: Uint) -> Uint {
    let (mut a, mut b) = (a, b);

    if a == 0 || b == 0 {
        return a | b;
    }

    let shift = (a | b).trailing_zeros();
    a >>= a.trailing_zeros();

    while b != 0 {
        b >>= b.trailing_zeros();
        if a > b {
            std::mem::swap(&mut a, &mut b)
        }
        b -= a;
    }

    a << shift
}

/// Get the least common multiple between `a` and `b`
///
/// # Examples
///
/// ```
/// assert_eq!(72, rsalgo::math::number_theory::lcm(24, 36));
/// ```
pub fn lcm(a: Uint, b: Uint) -> Uint {
    a / gcd(a, b) * b
}

/// Get the value of `base^exp % modulo`
///
/// # Examples
///
/// ```
/// assert_eq!(25, rsalgo::math::number_theory::quick_pow(5, 7, 100));
/// ```
pub fn quick_pow(base: Uint, exp: Uint, modulo: Uint) -> Uint {
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
pub fn quick_multiply(a: Uint, b: Uint, modulo: Uint) -> Uint {
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
        assert_eq!(3, super::gcd_euclid(3, 6));
        assert_eq!(12, super::gcd_euclid(24, 36));
        assert_eq!(5, super::gcd_euclid(5, 0));

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
