//! Lua diagnostic helpers.

use regex::Regex;

pub(crate) fn parse_location(message: &str) -> Option<(usize, usize)> {
    let pattern = Regex::new(r":([0-9]+)(?::([0-9]+))?:").ok()?;
    let captures = pattern.captures(message)?;
    let line = captures
        .get(1)?
        .as_str()
        .parse::<usize>()
        .ok()?
        .saturating_sub(1);
    let column = captures
        .get(2)
        .and_then(|capture| capture.as_str().parse::<usize>().ok())
        .unwrap_or(1)
        .saturating_sub(1);
    Some((line, column))
}
