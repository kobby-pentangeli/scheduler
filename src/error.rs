#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    FileCreateAndReadError(std::io::Error),

    #[error("{0}")]
    SerdeJsonSerializeError(serde_json::Error),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::FileCreateAndReadError(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::SerdeJsonSerializeError(value)
    }
}
