#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    SerdeJSONError(serde_json::Error),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeJSONError(value)
    }
}
