pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    SerdeJSONError(serde_json::Error),
    ParseIntError(std::num::ParseIntError),
    FromString(String),
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

impl From<std::num::ParseIntError> for Error {
    fn from(value: std::num::ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::FromString(value)
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::FromString(value.into())
    }
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Error::IOError(e) => format!("IOError: {e}"),
            Error::SerdeJSONError(e) => format!("SerdeJSONError: {e}"),
            Error::ParseIntError(e) => format!("ParseIntError: {e}"),
            Error::FromString(e) => e.to_string(),
        }
    }
}
