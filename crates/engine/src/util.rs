//! Lua diagnostic helpers.

use regex::Regex;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct DiagnosticSpan {
    pub start_line: usize,
    pub start_column: usize,
    pub end_line: usize,
    pub end_column: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ParserDiagnostic {
    pub span: DiagnosticSpan,
    pub message: String,
}

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

pub(crate) fn syntax_diagnostics(source: &str, message: &str) -> Vec<ParserDiagnostic> {
    let target_line = parse_location(message).map(|(line, _)| line).unwrap_or(0);
    let parsed = full_moon::parse_fallible(source, full_moon::LuaVersion::lua54());
    let mut diagnostics = parsed
        .errors()
        .iter()
        .map(|error| {
            let (start, end) = error.range();
            let start_line = start.line().saturating_sub(1);
            let end_line = end.line().saturating_sub(1);
            let distance = if target_line < start_line {
                start_line - target_line
            } else {
                target_line.saturating_sub(end_line)
            };
            let span = parser_span(source, start, end, target_line);
            (
                distance,
                start == end,
                ParserDiagnostic {
                    span,
                    message: error.error_message().into_owned(),
                },
            )
        })
        .collect::<Vec<_>>();

    diagnostics.sort_by_key(|(distance, empty, diagnostic)| {
        (
            *distance,
            *empty,
            diagnostic.span.start_line,
            diagnostic.span.start_column,
        )
    });
    diagnostics.dedup_by(|left, right| left.2 == right.2);

    if diagnostics.is_empty() {
        vec![ParserDiagnostic {
            span: line_span(source, target_line),
            message: message.to_owned(),
        }]
    } else {
        diagnostics
            .into_iter()
            .map(|(_, _, diagnostic)| diagnostic)
            .collect()
    }
}

fn parser_span(
    source: &str,
    start: full_moon::tokenizer::Position,
    end: full_moon::tokenizer::Position,
    fallback_line: usize,
) -> DiagnosticSpan {
    if start == end || start.line() == 0 || end.line() == 0 {
        let line = start.line().checked_sub(1).unwrap_or(fallback_line);
        return line_span(source, line);
    }

    let start_line = clamp_line(source, start.line().saturating_sub(1));
    let end_line = clamp_line(source, end.line().saturating_sub(1));
    let span = DiagnosticSpan {
        start_line,
        start_column: utf16_column(source, start_line, start.character().saturating_sub(1)),
        end_line,
        end_column: utf16_column(source, end_line, end.character().saturating_sub(1)),
    };

    if (span.start_line, span.start_column) < (span.end_line, span.end_column) {
        span
    } else {
        line_span(source, fallback_line)
    }
}

pub(crate) fn error_line_span(source: &str, message: &str) -> DiagnosticSpan {
    let line = parse_location(message).map(|(line, _)| line).unwrap_or(0);
    line_span(source, line)
}

fn line_span(source: &str, requested_line: usize) -> DiagnosticSpan {
    let lines = source_lines(source);
    let mut line = requested_line.min(lines.len().saturating_sub(1));

    if lines[line].is_empty() {
        if let Some(previous) = (0..line)
            .rev()
            .find(|candidate| !lines[*candidate].is_empty())
        {
            line = previous;
        } else if let Some(next) =
            (line + 1..lines.len()).find(|candidate| !lines[*candidate].is_empty())
        {
            line = next;
        }
    }

    DiagnosticSpan {
        start_line: line,
        start_column: 0,
        end_line: line,
        end_column: lines[line].encode_utf16().count(),
    }
}

fn clamp_line(source: &str, line: usize) -> usize {
    line.min(source_lines(source).len().saturating_sub(1))
}

fn utf16_column(source: &str, line: usize, character_column: usize) -> usize {
    source_lines(source)[line]
        .chars()
        .take(character_column)
        .map(char::len_utf16)
        .sum()
}

fn source_lines(source: &str) -> Vec<&str> {
    source
        .split('\n')
        .map(|line| line.strip_suffix('\r').unwrap_or(line))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{error_line_span, syntax_diagnostics};

    #[test]
    fn uses_full_moon_token_range() {
        let source = "function check()\n  local value =\nend";
        let diagnostics = syntax_diagnostics(source, "script:2: unexpected symbol near 'end'");
        let span = diagnostics[0].span;

        assert_eq!((span.start_line, span.start_column), (1, 14));
        assert_eq!((span.end_line, span.end_column), (1, 15));
    }

    #[test]
    fn keeps_multiline_parser_ranges() {
        let source = "function check()\n  return \"unterminated\nend";
        let diagnostics =
            syntax_diagnostics(source, "script:2: unfinished string near '\"unterminated'");
        let span = diagnostics[0].span;

        assert_eq!(span.start_line, 1);
        assert_eq!(span.end_line, 2);
    }

    #[test]
    fn expands_zero_width_parser_ranges_to_a_line() {
        let source = "function check(";
        let diagnostics =
            syntax_diagnostics(source, "script:1: <name> or '...' expected near <eof>");
        let span = diagnostics[0].span;

        assert_eq!((span.start_line, span.start_column), (0, 0));
        assert_eq!((span.end_line, span.end_column), (0, source.len()));
    }

    #[test]
    fn returns_multiple_parser_errors() {
        let source = "function check()\n  if true then\n    return true\nfunction generate() end";
        let diagnostics = syntax_diagnostics(source, "script:5: 'end' expected near <eof>");

        assert_eq!(diagnostics.len(), 2);
        assert_ne!(diagnostics[0], diagnostics[1]);
    }

    #[test]
    fn runtime_ranges_cover_the_reported_line() {
        let source = "local value = nil + 1\nfunction check() end";
        let span = error_line_span(
            source,
            "script:1: attempt to perform arithmetic on a nil value",
        );

        assert_eq!((span.start_line, span.start_column), (0, 0));
        assert_eq!((span.end_line, span.end_column), (0, 21));
    }

    #[test]
    fn columns_use_codemirror_utf16_offsets() {
        let source = "local text = '😀' +";
        let span = error_line_span(source, "script:1: error");

        assert_eq!(span.end_column, source.encode_utf16().count());
    }
}
