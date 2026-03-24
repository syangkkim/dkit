/// Path evaluator — navigates a [`Value`](crate::value::Value) tree using parsed paths.
pub mod evaluator;
/// Pipeline filter operations (where, sort, limit, aggregation, etc.).
pub mod filter;
/// Built-in query functions (string, math, date, type conversion).
pub mod functions;
/// Query parser — converts query strings into an AST.
pub mod parser;
