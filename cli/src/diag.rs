use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::Files;
use codespan_reporting::term::{self, termcolor::WriteColor};
use reflexo_typst::{DiagnosticFormat, Result};
use typst::diag::{eco_format, Severity, SourceDiagnostic};
use typst::syntax::{FileId, Span};
use typst::{World, WorldExt};

/// Prints diagnostic messages to the terminal.
pub fn print_diagnostics<'d, 'files, W: World + Files<'files, FileId = FileId>>(
    world: &'files W,
    diagnostics: impl Iterator<Item = &'d SourceDiagnostic>,
    diagnostic_format: DiagnosticFormat,
    w: &mut dyn WriteColor,
) -> Result<(), codespan_reporting::files::Error> {
    let mut config = term::Config {
        tab_width: 2,
        ..Default::default()
    };
    if diagnostic_format == DiagnosticFormat::Short {
        config.display_style = term::DisplayStyle::Short;
    }

    for diagnostic in diagnostics {
        let diag = match diagnostic.severity {
            Severity::Error => Diagnostic::error(),
            Severity::Warning => Diagnostic::warning(),
        }
        .with_message(diagnostic.message.clone())
        .with_notes(
            diagnostic
                .hints
                .iter()
                .map(|e| (eco_format!("hint: {e}")).into())
                .collect(),
        )
        .with_labels(label(world, diagnostic.span).into_iter().collect());

        term::emit(w, &config, world, &diag)?;

        // Stacktrace-like helper diagnostics.
        for point in &diagnostic.trace {
            let message = point.v.to_string();
            let help = Diagnostic::help()
                .with_message(message)
                .with_labels(label(world, point.span).into_iter().collect());

            term::emit(w, &config, world, &help)?;
        }
    }

    Ok(())
}

/// Creates a label for a span.
fn label<'files, W: World + Files<'files, FileId = FileId>>(
    world: &'files W,
    span: Span,
) -> Option<Label<FileId>> {
    Some(Label::primary(span.id()?, world.range(span)?))
}
