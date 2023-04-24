pub mod error;
pub mod task_manager;

pub use task_manager::TaskManager;

pub type Result<T> = std::result::Result<T, error::Error>;
