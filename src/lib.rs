pub mod error;
pub mod todo;

pub use todo::Todo;

pub type Result<T> = std::result::Result<T, error::Error>;
