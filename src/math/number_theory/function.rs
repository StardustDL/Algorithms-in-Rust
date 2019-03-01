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
    let (x, _, d) = super::exgcd(value, modulus);
    if d == 1 {
        Some(modulo(x, modulus))
    } else {
        None
    }
}

// log_base(value) baby_step_giant_step
pub fn log(base: Uint, value: Uint, modulus: Uint) -> Option<Uint> {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    let m = (modulus as f64).sqrt().ceil() as Uint;
    let mut e = 1;
    map.insert(e, 0);
    for i in 1..m {
        e = e * base % modulus;
        map.entry(e).or_insert(i);
    }
    let v = match inverse(super::quick_pow(base, m, modulus), modulus) {
        Some(x) => x,
        None => return None,
    };
    let mut vt = value;
    for i in 0..m {
        match map.entry(vt) {
            std::collections::hash_map::Entry::Occupied(oc) => return Some((i * m) + *oc.get()),
            std::collections::hash_map::Entry::Vacant(_) => (),
        }
        vt = vt * v % modulus;
    }
    None
}
