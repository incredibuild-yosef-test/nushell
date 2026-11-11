mod capitalize;
mod downcase;
mod str_;
mod upcase;

pub use capitalize::StrCapitalize;
pub use downcase::StrDowncase;
pub use downcase::StrLowercase;
pub use str_::Str;
pub use upcase::StrUpcase;
pub use upcase::StrUppercase;

use nu_cmd_base::input_handler::{CmdArgument, operate as general_operate};
use nu_engine::command_prelude::*;

/// Arguments shared by the `str` case commands: the conversion and the cell paths to apply it to.
struct Arguments<F: Fn(&str) -> String + Send + Sync + 'static> {
    case_operation: &'static F,
    cell_paths: Option<Vec<CellPath>>,
}

impl<F: Fn(&str) -> String + Send + Sync + 'static> CmdArgument for Arguments<F> {
    fn take_cell_paths(&mut self) -> Option<Vec<CellPath>> {
        self.cell_paths.take()
    }
}

/// Runs a `str` case command: applies `case_operation` to every string in `input`, or only
/// to the given cell paths.
pub fn operate<F>(
    engine_state: &EngineState,
    stack: &mut Stack,
    call: &Call,
    input: PipelineData,
    case_operation: &'static F,
) -> Result<PipelineData, ShellError>
where
    F: Fn(&str) -> String + Send + Sync + 'static,
{
    let cell_paths: Vec<CellPath> = call.rest(engine_state, stack, 0)?;
    let cell_paths = (!cell_paths.is_empty()).then_some(cell_paths);
    let args = Arguments {
        case_operation,
        cell_paths,
    };
    general_operate(action, args, input, call.head, engine_state.signals())
}

/// Converts one value; non-strings become an "only supports string" error.
fn action<F>(input: &Value, args: &Arguments<F>, head: Span) -> Value
where
    F: Fn(&str) -> String + Send + Sync + 'static,
{
    let case_operation = args.case_operation;
    match input {
        Value::String { val, .. } => Value::string(case_operation(val), head),
        Value::Error { .. } => input.clone(),
        _ => Value::error(
            ShellError::OnlySupportsThisInputType {
                exp_input_type: "string".into(),
                wrong_type: input.get_type().to_string(),
                dst_span: head,
                src_span: input.span(),
            },
            head,
        ),
    }
}
