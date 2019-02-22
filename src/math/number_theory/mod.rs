//! Number theory related algorithms

pub type Uint = usize;
pub type Int = isize;

mod gcd;
pub use gcd::*;

mod prime;
pub use prime::*;

mod function;
pub use function::*;

/// Get the value of `base^exp % modulus`
///
/// # Examples
///
/// ```
/// assert_eq!(25, rsalgo::math::number_theory::quick_pow(5, 7, 100));
/// ```
pub fn quick_pow(base: Uint, exp: Uint, modulus: Uint) -> Uint {
    let mut ans = 1;
    let (mut base, mut exp) = (base % modulus, exp);
    while exp > 0 {
        if exp % 2 != 0 {
            ans *= base;
            ans %= modulus;
        }
        base *= base;
        base %= modulus;
        exp /= 2;
    }
    ans
}

/// Get the value of `base*exp % modulus`
///
/// # Examples
///
/// ```
/// assert_eq!(5, rsalgo::math::number_theory::quick_multiply(5, 7, 10));
/// ```
pub fn quick_multiply(a: Uint, b: Uint, modulus: Uint) -> Uint {
    let mut ans = 0;
    let (mut a, mut b) = (a % modulus, b % modulus);
    while b > 0 {
        if b % 2 != 0 {
            ans += a;
            ans %= modulus;
        }
        a += a;
        a %= modulus;
        b /= 2;
    }
    ans
}

#[cfg(test)]
mod tests{
    use rand::Rng;

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