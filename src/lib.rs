pub mod error;
pub mod journal;

pub use journal::ToDo;

pub type Result<T> = std::result::Result<T, error::Error>;
