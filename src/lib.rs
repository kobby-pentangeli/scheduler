pub mod error;
pub mod journal;

pub use journal::Journal;

pub type Result<T> = std::result::Result<T, error::Error>;
