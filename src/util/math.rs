use std::f64::consts::E;

/// curve is a function that calculates the value of a curve given the parameters s, r, d, and x.
///
/// - "s" is the maximum value.
/// - "r" is the maximum value.
/// - "d" is the degree of difficulty of the challenge.
/// - "x" is the quantity of correct submissions.
pub fn curve(s: i64, r: i64, d: i64, x: i64) -> i64 {
    let ratio = r as f64 / s as f64;
    let result =
        (s as f64 * (ratio + (1.0 - ratio) * E.powf((1.0 - x as f64) / d as f64))).floor() as i64;
    return result.min(s);
}
