use super::{Int, Uint};

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

// return (x,y,d) ax+by=d=gcd(a,b)
pub fn exgcd(a: Uint, b: Uint) -> (Int, Int, Uint) {
    if b == 0 {
        (1, 0, a)
    } else {
        let (x, y, d) = exgcd(b, a % b);
        (y, x - ((a / b) as Int) * y, d)
    }
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
}
