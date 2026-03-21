use std::f64::consts::E;

/// Curve used for dynamic scoring from max/min pts, difficulty, and solve
/// count.
pub fn curve(s: i64, r: i64, d: i64, x: i64) -> i64 {
    let ratio = r as f64 / s as f64;
    let result =
        (s as f64 * (ratio + (1.0 - ratio) * E.powf((1.0 - x as f64) / d as f64))).floor() as i64;
    result.min(s)
}
