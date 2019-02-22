use super::{Int, Uint};

// O(sqrt(n))
pub fn phi(n: Uint) -> Uint {
    let mut n = n;
    let mut ans = n;
    let rt = (n as f64).sqrt().floor() as Uint;
    for i in 2..=rt {
        if n % i == 0 {
            ans = ans / i * (i - 1);
            while n % i == 0 {
                n /= i;
            }
        }
    }
    if n > 1 {
        ans = ans / n * (n - 1);
    }
    ans
}

// Return (b_i, e_i): n=PHI{ b_i^e_i }
pub fn decompose(n: Uint) -> Vec<(Uint, Uint)> {
    let mut n = n;
    let mut res = Vec::new();
    let rt = (n as f64).sqrt().floor() as Uint;
    for i in 2..=rt {
        if n % i == 0 {
            let mut e = 0;
            while n % i == 0 {
                n /= i;
                e += 1;
            }
            res.push((i, e));
        }
    }
    if n > 1 {
        res.push((n, 1));
    }
    res
}

pub fn modulo(value: Int, modulus: Uint) -> Uint {
    if value.is_negative() {
        ((value % (modulus as Int)) + modulus as Int) as Uint
    } else {
        (value as Uint) % modulus
    }
}

pub fn inverse(value: Uint, modulus: Uint) -> Option<Uint> {
    let (x, y, d) = super::exgcd(value, modulus);
    if d == 1 {
        Some(modulo(x, modulus))
    } else {
        None
    }
}
