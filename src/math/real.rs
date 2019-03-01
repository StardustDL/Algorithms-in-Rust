pub fn trichotomy<O, F>(lower: f64, upper: f64, eps: f64, func: F) -> f64
where
    O: PartialOrd,
    F: Fn(f64) -> O,
{
    let mut lower = lower;
    let mut upper = upper;
    while lower + eps < upper {
        let delta = (upper - lower) / 3.0;
        let ml = lower + delta;
        let mr = upper - delta;

        if func(ml) < func(mr) {
            upper = mr;
        } else {
            lower = ml;
        }
    }
    lower
}
