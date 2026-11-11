use nu_parser::parse;
use nu_protocol::{
    ParseError,
    engine::{EngineState, StateWorkingSet},
};
use reedline::{ValidationResult, Validator};
use std::sync::Arc;

/// Tells the line editor whether the line typed so far is a complete command.
///
/// An incomplete line (for example an unclosed `{`) makes Enter insert a newline
/// instead of submitting.
pub struct NuValidator {
    pub engine_state: Arc<EngineState>,
}

impl Validator for NuValidator {
    /// Parses `line` in a scratch working set; nothing is added to the engine state.
    fn validate(&self, line: &str) -> ValidationResult {
        let mut working_set = StateWorkingSet::new(&self.engine_state);
        parse(&mut working_set, None, line.as_bytes(), false);

        if is_incomplete(&working_set.parse_errors) {
            ValidationResult::Incomplete
        } else {
            ValidationResult::Complete
        }
    }
}

/// Whether a line that parsed with `errors` is unfinished rather than wrong.
///
/// Unclosed delimiters and unexpected EOF both mean the user may still be
/// typing a multi-line construct (e.g. an open `{` in the REPL). Unbalanced
/// closers are complete but wrong inputs, so the REPL submits the line and
/// shows the diagnostic.
pub(crate) fn is_incomplete(errors: &[ParseError]) -> bool {
    matches!(
        errors.first(),
        Some(ParseError::UnexpectedEof(..) | ParseError::Unclosed(..))
    )
}
