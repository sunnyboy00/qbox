pub mod app;
pub mod assets;
pub mod bus;
pub mod counter;
pub mod quotes;
pub mod risk;
pub mod strategy;
pub mod trade;

use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum QboxError {
    #[error("data store disconnected")]
    Disconnect,
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
}

#[derive(Debug, Clone)]
pub enum LogEvent {
    Error(QboxError),
}
