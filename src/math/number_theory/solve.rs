use super::Uint;

// ax=b(mod c)
pub fn linear_modulo(a: Uint, b: Uint, c: Uint) -> Option<Uint> {
    let (x, _, d) = super::exgcd(a, c);
    if b % d == 0 {
        Some(super::modulo(x, c))
    } else {
        None
    }
}
