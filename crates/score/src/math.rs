//! Pure numeric helpers for dynamic CTF challenge scoring.

use std::f64::consts::E;

/// Computes the decaying base score between `min_pts` (floor) and `max_pts`
/// (ceiling) from `solve_count`.
///
/// - `max_pts` (`s`): initial points when 0 or 1 teams have solved.
/// - `min_pts` (`r`): asymptotic floor as solves grow.
/// - `difficulty` (`d`): decay scale; larger `d` slows decay.
/// - `solve_count` (`x`): number of valid (`Passed`-team) solves.
pub fn curve(max_pts: i64, min_pts: i64, difficulty: i64, solve_count: i64) -> i64 {
    if max_pts <= 0 {
        return 0;
    }
    if difficulty <= 0 || solve_count <= 1 {
        return max_pts;
    }
    let ratio = min_pts as f64 / max_pts as f64;
    let result = (max_pts as f64
        * (ratio + (1.0 - ratio) * E.powf((1.0 - solve_count as f64) / difficulty as f64)))
    .floor() as i64;
    result.min(max_pts)
}

#[cfg(test)]
mod tests {
    use super::curve;

    #[test]
    fn curve_matches_expected_decay_boundaries() {
        assert_eq!(curve(1_000, 100, 10, 0), 1_000);
        assert_eq!(curve(1_000, 100, 10, 1), 1_000);
        assert_eq!(curve(1_000, 1_000, 10, 50), 1_000);
        assert!(curve(1_000, 100, 10, 2) < 1_000);
        assert!(curve(1_000, 100, 10, 100) >= 100);
        assert_eq!(curve(0, 0, 10, 5), 0);
    }
}
