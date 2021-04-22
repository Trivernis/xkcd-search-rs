use thiserror::Error;

pub type XKCDResult<T> = Result<T, XKCDError>;

#[derive(Debug, Error)]
pub enum XKCDError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error("Parse Error: {0}")]
    ParseError(String),
}

impl From<&str> for XKCDError {
    fn from(s: &str) -> Self {
        Self::ParseError(s.to_string())
    }
}
