use std::fmt;

/// Error types used throughout this crate.
///
/// The `Display` impl for `Error` will provide information about the error
/// itself. For more detailed information about the error, including the program
/// context in which it occurred, see
/// [`State.full_error_message_with_context()`](struct.State.html#method.full_error_message_with_context).
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Error {
    //syncxxx-1-14
    PathComplete,
    /// While performing an operation, we discovered the current path is unsat.
    ///
    /// This error type is used internally, but (by default) isn't exposed to consumers of `ExecutionManager`;
    /// see [`Config.squash_unsats`](config/struct.Config.html#structfield.squash_unsats).
    Unsat,
    /// The current path has exceeded the configured `loop_bound` (see [`Config`](config/struct.Config.html)).
    /// (The `usize` here indicates the value of the configured `loop_bound`.)
    LoopBoundExceeded(usize),
    /// The current path has attempted to dereference a null pointer (or
    /// more precisely, a pointer for which `NULL` is a possible value)
    NullPointerDereference,
    /// Processing a call of a function with the given name, but failed to find an LLVM definition, a function hook, or a built-in handler for it
    FunctionNotFound(String),
    /// The solver returned this processing error while evaluating a query.
    /// Often, this is a timeout; see [`Config.solver_query_timeout`](config/struct.Config.html#structfield.solver_query_timeout)
    SolverError(String),
    /// Encountered an LLVM instruction which is not currently supported
    UnsupportedInstruction(String),
    /// Encountered an LLVM instruction which was malformed, or at least didn't conform to our expected invariants
    MalformedInstruction(String),
    /// Reached an LLVM `Unreachable` instruction
    UnreachableInstruction,
    /// Failed to interpret some symbolic value (`BV`) as a function pointer,
    /// because it has a possible solution (the `u64` here) which points to
    /// something that's not a function
    FailedToResolveFunctionPointer(u64),
    /// The hook for some function returned a value which didn't match the
    /// function return type: for instance, a value of the wrong size.
    /// The `String` here just describes the error
    HookReturnValueMismatch(String),
    /// Some kind of error which doesn't fall into one of the above categories.
    /// The `String` here describes the error
    OtherError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // syncxxx-1-14
            Error::PathComplete =>
                write!(f, "`PathComplete`: hahaha"),
            Error::Unsat =>
                write!(f, "`Unsat`: the current state or path is unsat"),
            Error::LoopBoundExceeded(bound) =>
                write!(f, "`LoopBoundExceeded`: the current path has exceeded the configured `loop_bound`, which was {}", bound),
            Error::NullPointerDereference =>
                write!(f, "`NullPointerDereference`: the current path has attempted to dereference a null pointer"),
            Error::FunctionNotFound(funcname) =>
                write!(f, "`FunctionNotFound`: encountered a call of a function named {:?}, but failed to find an LLVM definition, a function hook, or a built-in handler for it", funcname),
            Error::SolverError(details) =>
                write!(f, "`SolverError`: the solver returned this error while evaluating a query: {}", details),
            Error::UnsupportedInstruction(details) =>
                write!(f, "`UnsupportedInstruction`: encountered an LLVM instruction which is not currently supported: {}", details),
            Error::MalformedInstruction(details) =>
                write!(f, "`MalformedInstruction`: encountered an LLVM instruction which was malformed, or at least didn't conform to our expected invariants: {}", details),
            Error::UnreachableInstruction =>
                write!(f, "`UnreachableInstruction`: Reached an LLVM 'Unreachable' instruction"),
            Error::FailedToResolveFunctionPointer(solution) =>
                write!(f, "`FailedToResolveFunctionPointer`: Can't resolve a symbolically-valued function pointer, because one possible solution for it ({:#x}) points to something that's not a function", solution),
            Error::HookReturnValueMismatch(details) =>
                write!(f, "`HookReturnValueMismatch`: {}", details),
            Error::OtherError(details) =>
                write!(f, "`OtherError`: {}", details),
        }
    }
}

impl From<Error> for String {
    fn from(e: Error) -> String {
        e.to_string() // use the Display impl
    }
}

/// A type alias for convenience, similar to how `std::io::Result` is used for I/O operations
pub type Result<T> = std::result::Result<T, Error>;
