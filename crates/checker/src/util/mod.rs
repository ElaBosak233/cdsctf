use rune::{
    Sources,
    ast::Spanned,
    diagnostics::{Diagnostic, FatalDiagnosticKind},
};

use crate::traits::{DiagnosticKind, DiagnosticMarker};

pub mod leet;
pub mod suid;
pub mod xxtea;

pub(crate) fn diagnostic_to_marker(
    diagnostic: &Diagnostic,
    sources: &Sources,
) -> Option<DiagnosticMarker> {
    match diagnostic {
        Diagnostic::Fatal(fatal) => {
            if let FatalDiagnosticKind::CompileError(error) = fatal.kind() {
                let span = error.span();
                let source = sources.get(fatal.source_id())?;
                let (start_line, start_column) =
                    source.pos_to_utf8_linecol(span.start.into_usize());
                let (end_line, end_column) = source.pos_to_utf8_linecol(span.end.into_usize());
                return Some(DiagnosticMarker {
                    kind: DiagnosticKind::Error,
                    message: error.to_string(),
                    start_line,
                    start_column,
                    end_line,
                    end_column,
                });
            }
        }
        Diagnostic::Warning(warning) => {
            let span = warning.span();
            let source = sources.get(warning.source_id())?;
            let (start_line, start_column) = source.pos_to_utf8_linecol(span.start.into_usize());
            let (end_line, end_column) = source.pos_to_utf8_linecol(span.end.into_usize());
            return Some(DiagnosticMarker {
                kind: DiagnosticKind::Warning,
                message: warning.to_string(),
                start_line,
                start_column,
                end_line,
                end_column,
            });
        }
        _ => {}
    }
    None
}
