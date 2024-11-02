use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("Cell length must match with initialized value!")]
    CellLengthMismatch,
    #[error("Invalid regex expression {0}")]
    RegexError(#[from] regex::Error),
}
