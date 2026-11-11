use crate::{
    eval_block, eval_block_with_early_return, eval_expression, eval_expression_with_input,
    eval_ir_block, eval_subexpression,
};
use nu_protocol::{
    PipelineData, PipelineExecutionData, ShellError, Value,
    ast::{Block, Expression},
    debugger::{WithDebug, WithoutDebug},
    engine::{EngineState, Stack},
};

/// Picks the `WithDebug` or `WithoutDebug` instantiation of an eval function, depending on
/// whether `engine_state` has a debugger attached.
macro_rules! with_debugger {
    ($engine_state:expr, $eval_fn:ident) => {
        if $engine_state.is_debugging() {
            $eval_fn::<WithDebug>
        } else {
            $eval_fn::<WithoutDebug>
        }
    };
}

/// Type of eval_block() function
pub type EvalBlockFn =
    fn(&EngineState, &mut Stack, &Block, PipelineData) -> Result<PipelineExecutionData, ShellError>;

/// Type of eval_ir_block() function
pub type EvalIrBlockFn =
    fn(&EngineState, &mut Stack, &Block, PipelineData) -> Result<PipelineExecutionData, ShellError>;

/// Type of eval_block_with_early_return() function
pub type EvalBlockWithEarlyReturnFn =
    fn(&EngineState, &mut Stack, &Block, PipelineData) -> Result<PipelineExecutionData, ShellError>;

/// Type of eval_expression() function
pub type EvalExpressionFn = fn(&EngineState, &mut Stack, &Expression) -> Result<Value, ShellError>;

/// Type of eval_expression_with_input() function
pub type EvalExpressionWithInputFn =
    fn(&EngineState, &mut Stack, &Expression, PipelineData) -> Result<PipelineData, ShellError>;

/// Type of eval_subexpression() function
pub type EvalSubexpressionFn =
    fn(&EngineState, &mut Stack, &Block, PipelineData) -> Result<PipelineData, ShellError>;

/// Helper function to fetch `eval_block()` with the correct type parameter based on whether
/// engine_state is configured with or without a debugger.
pub fn get_eval_block(engine_state: &EngineState) -> EvalBlockFn {
    with_debugger!(engine_state, eval_block)
}

/// Helper function to fetch `eval_ir_block()` with the correct type parameter based on whether
/// engine_state is configured with or without a debugger.
pub fn get_eval_ir_block(engine_state: &EngineState) -> EvalIrBlockFn {
    with_debugger!(engine_state, eval_ir_block)
}

/// Helper function to fetch `eval_block_with_early_return()` with the correct type parameter based
/// on whether engine_state is configured with or without a debugger.
pub fn get_eval_block_with_early_return(engine_state: &EngineState) -> EvalBlockWithEarlyReturnFn {
    with_debugger!(engine_state, eval_block_with_early_return)
}

/// Helper function to fetch `eval_expression()` with the correct type parameter based on whether
/// engine_state is configured with or without a debugger.
pub fn get_eval_expression(engine_state: &EngineState) -> EvalExpressionFn {
    with_debugger!(engine_state, eval_expression)
}

/// Helper function to fetch `eval_expression_with_input()` with the correct type parameter based
/// on whether engine_state is configured with or without a debugger.
pub fn get_eval_expression_with_input(engine_state: &EngineState) -> EvalExpressionWithInputFn {
    with_debugger!(engine_state, eval_expression_with_input)
}

/// Helper function to fetch `eval_subexpression()` with the correct type parameter based on whether
/// engine_state is configured with or without a debugger.
pub fn get_eval_subexpression(engine_state: &EngineState) -> EvalSubexpressionFn {
    with_debugger!(engine_state, eval_subexpression)
}
