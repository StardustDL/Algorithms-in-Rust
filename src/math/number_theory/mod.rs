//! Number theory related algorithms

pub type Uint = usize;
pub type Int = isize;

mod gcd;
pub use gcd::*;

mod prime;
pub use prime::*;
