//! Pure numeric helpers for dynamic scoring in [`super::calculate`].

use std::f64::consts::E;

/// Computes a **decaying base score** between `r` (floor) and `s` (ceiling)
/// from solve count `x`.
///
/// - `s` — maximum points (starting value when few have solved).
/// - `r` — minimum points (asymptotic floor; ratio `r/s` blends into the
///   formula).
/// - `d` — difficulty scale: larger `d` slows the decay as `x` increases.
/// - `x` — number of correct solves (including the current wave).
///
/// The implementation blends `r/s` with an exponential in `(1 - x) / d`, then
/// clamps to `s`.
pub fn curve(s: i64, r: i64, d: i64, x: i64) -> i64 {
    let ratio = r as f64 / s as f64;
    let result =
        (s as f64 * (ratio + (1.0 - ratio) * E.powf((1.0 - x as f64) / d as f64))).floor() as i64;
    result.min(s)
}
