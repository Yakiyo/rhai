use std::error::Error;

use crate::any::Dynamic;
use crate::error::ParseError;
use crate::parser::Position;

/// Evaluation result.
///
/// All wrapped `Position` values represent the location in the script where the error occurs.
#[derive(Debug)]
pub enum EvalAltResult {
    /// Syntax error.
    ErrorParsing(ParseError),
    /// Call to an unknown function. Wrapped value is the name of the function.
    ErrorFunctionNotFound(String, Position),
    /// Function call has incorrect number of arguments.
    /// Wrapped values are the name of the function, the number of parameters required
    /// and the actual number of arguments passed.
    ErrorFunctionArgsMismatch(String, usize, usize, Position),
    /// Non-boolean operand encountered for boolean operator. Wrapped value is the operator.
    ErrorBooleanArgMismatch(String, Position),
    /// Array access out-of-bounds.
    /// Wrapped values are the current number of elements in the array and the index number.
    ErrorArrayBounds(usize, i64, Position),
    /// String indexing out-of-bounds.
    /// Wrapped values are the current number of characters in the string and the index number.
    ErrorStringBounds(usize, i64, Position),
    /// Trying to index into a type that is not an array and not a string.
    ErrorIndexingType(Position),
    /// Trying to index into an array or string with an index that is not `i64`.
    ErrorIndexExpr(Position),
    /// The guard expression in an `if` statement does not return a boolean value.
    ErrorIfGuard(Position),
    /// The `for` statement encounters a type that is not an iterator.
    ErrorFor(Position),
    /// Usage of an unknown variable. Wrapped value is the name of the variable.
    ErrorVariableNotFound(String, Position),
    /// Assignment to an inappropriate LHS (left-hand-side) expression.
    ErrorAssignmentToUnknownLHS(Position),
    /// Returned type is not the same as the required output type.
    /// Wrapped value is the type of the actual result.
    ErrorMismatchOutputType(String, Position),
    /// Error reading from a script file. Wrapped value is the path of the script file.
    ErrorReadingScriptFile(String, std::io::Error),
    /// Inappropriate member access.
    ErrorDotExpr(Position),
    /// Arithmetic error encountered. Wrapped value is the error message.
    ErrorArithmetic(String, Position),
    /// Run-time error encountered. Wrapped value is the error message.
    ErrorRuntime(String, Position),
    /// Internal use: Breaking out of loops.
    LoopBreak,
    /// Not an error: Value returned from a script via the `return` keyword.
    /// Wrapped value is the result value.
    Return(Dynamic, Position),
}

impl Error for EvalAltResult {
    fn description(&self) -> &str {
        match self {
            Self::ErrorParsing(p) => p.description(),
            Self::ErrorFunctionNotFound(_, _) => "Function not found",
            Self::ErrorFunctionArgsMismatch(_, _, _, _) => {
                "Function call with wrong number of arguments"
            }
            Self::ErrorBooleanArgMismatch(_, _) => "Boolean operator expects boolean operands",
            Self::ErrorIndexExpr(_) => "Indexing into an array or string expects an integer index",
            Self::ErrorIndexingType(_) => "Indexing can only be performed on an array or a string",
            Self::ErrorArrayBounds(_, index, _) if *index < 0 => {
                "Array access expects non-negative index"
            }
            Self::ErrorArrayBounds(max, _, _) if *max == 0 => "Access of empty array",
            Self::ErrorArrayBounds(_, _, _) => "Array index out of bounds",
            Self::ErrorStringBounds(_, index, _) if *index < 0 => {
                "Indexing a string expects a non-negative index"
            }
            Self::ErrorStringBounds(max, _, _) if *max == 0 => "Indexing of empty string",
            Self::ErrorStringBounds(_, _, _) => "String index out of bounds",
            Self::ErrorIfGuard(_) => "If guard expects boolean expression",
            Self::ErrorFor(_) => "For loop expects array or range",
            Self::ErrorVariableNotFound(_, _) => "Variable not found",
            Self::ErrorAssignmentToUnknownLHS(_) => {
                "Assignment to an unsupported left-hand side expression"
            }
            Self::ErrorMismatchOutputType(_, _) => "Output type is incorrect",
            Self::ErrorReadingScriptFile(_, _) => "Cannot read from script file",
            Self::ErrorDotExpr(_) => "Malformed dot expression",
            Self::ErrorArithmetic(_, _) => "Arithmetic error",
            Self::ErrorRuntime(_, _) => "Runtime error",
            Self::LoopBreak => "[Not Error] Breaks out of loop",
            Self::Return(_, _) => "[Not Error] Function returns value",
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

impl std::fmt::Display for EvalAltResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = self.description();

        match self {
            Self::ErrorFunctionNotFound(s, pos) => write!(f, "{}: '{}' ({})", desc, s, pos),
            Self::ErrorVariableNotFound(s, pos) => write!(f, "{}: '{}' ({})", desc, s, pos),
            Self::ErrorIndexingType(pos) => write!(f, "{} ({})", desc, pos),
            Self::ErrorIndexExpr(pos) => write!(f, "{} ({})", desc, pos),
            Self::ErrorIfGuard(pos) => write!(f, "{} ({})", desc, pos),
            Self::ErrorFor(pos) => write!(f, "{} ({})", desc, pos),
            Self::ErrorAssignmentToUnknownLHS(pos) => write!(f, "{} ({})", desc, pos),
            Self::ErrorMismatchOutputType(s, pos) => write!(f, "{}: {} ({})", desc, s, pos),
            Self::ErrorDotExpr(pos) => write!(f, "{} ({})", desc, pos),
            Self::ErrorArithmetic(s, pos) => write!(f, "{}: {} ({})", desc, s, pos),
            Self::ErrorRuntime(s, pos) if s.is_empty() => write!(f, "{} ({})", desc, pos),
            Self::ErrorRuntime(s, pos) => write!(f, "{}: {} ({})", desc, s, pos),
            Self::LoopBreak => write!(f, "{}", desc),
            Self::Return(_, pos) => write!(f, "{} ({})", desc, pos),
            Self::ErrorReadingScriptFile(filename, err) => {
                write!(f, "{} '{}': {}", desc, filename, err)
            }
            Self::ErrorParsing(p) => write!(f, "Syntax error: {}", p),
            Self::ErrorFunctionArgsMismatch(fun, need, n, pos) => write!(
                f,
                "Function '{}' expects {} argument(s) but {} found ({})",
                fun, need, n, pos
            ),
            Self::ErrorBooleanArgMismatch(op, pos) => {
                write!(f, "{} operator expects boolean operands ({})", op, pos)
            }
            Self::ErrorArrayBounds(_, index, pos) if *index < 0 => {
                write!(f, "{}: {} < 0 ({})", desc, index, pos)
            }
            Self::ErrorArrayBounds(max, _, pos) if *max == 0 => write!(f, "{} ({})", desc, pos),
            Self::ErrorArrayBounds(max, index, pos) => write!(
                f,
                "Array index {} is out of bounds: only {} element{} in the array ({})",
                index,
                max,
                if *max > 1 { "s" } else { "" },
                pos
            ),
            Self::ErrorStringBounds(_, index, pos) if *index < 0 => {
                write!(f, "{}: {} < 0 ({})", desc, index, pos)
            }
            Self::ErrorStringBounds(max, _, pos) if *max == 0 => write!(f, "{} ({})", desc, pos),
            Self::ErrorStringBounds(max, index, pos) => write!(
                f,
                "String index {} is out of bounds: only {} character{} in the string ({})",
                index,
                max,
                if *max > 1 { "s" } else { "" },
                pos
            ),
        }
    }
}

impl From<ParseError> for EvalAltResult {
    fn from(err: ParseError) -> Self {
        Self::ErrorParsing(err)
    }
}
