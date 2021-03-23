//! Used for describing errors found during transpilation.


/// Categories of transpilation errors.
pub enum TranspileErrorKind {
    /// The `opinionated_rust_to_typescript` library does not currently
    /// implement the transpilation specified in `config`.
    ConfigNotImplemented,
    /// Fallback, when no other error fits.
    UnknownError,
}

impl TranspileErrorKind {
    /// 
    pub fn to_string(&self) -> &str {
        match self {
            Self::ConfigNotImplemented => "ConfigNotImplemented",
            Self::UnknownError => "UnknownError",
        }
    }
}

/// Encapsulates an error found during transpilation.
/// 
/// Many errors may be encountered while transpiling a given Rust program. These
/// are converted into `TranspileError`s, and recorded in the `errors` vector of
/// the [`TranspileResult`](super::result::TranspileResult).
pub struct TranspileError {
    /// The character position within the line where the error occurred, or 0.
    pub column: usize,
    /// Broad category of the error.
    pub kind: TranspileErrorKind,
    /// The line number of the Rust code which caused the error, or 0.
    pub line_number: usize,
    /// A short explanation of the error, to help a developer debug it.
    pub message: &'static str,
}
