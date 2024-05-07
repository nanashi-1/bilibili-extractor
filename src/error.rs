pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Serde JSON error: {0}")]
    SerdeJSONError(#[from] serde_json::Error),

    #[error("Parse Integer error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("Bilibili Extractor error: {0}")]
    BilibiliExtractorError(String),
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::BilibiliExtractorError(s.to_string())
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::BilibiliExtractorError(s)
    }
}
