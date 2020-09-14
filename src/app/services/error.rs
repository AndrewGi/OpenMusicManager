#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Reqwest(reqwest::Error),
    InvalidHeaderValue(reqwest::header::InvalidHeaderValue),
    URLParse(url::ParseError),
    JSON(serde_json::Error),
    URLSerialization(serde_urlencoded::ser::Error),
    ChannelError,
    Other(String),
}
impl<T> From<tokio::sync::mpsc::error::SendError<T>> for Error {
    fn from(_: tokio::sync::mpsc::error::SendError<T>) -> Self {
        Error::ChannelError
    }
}
impl From<serde_urlencoded::ser::Error> for Error {
    fn from(e: serde_urlencoded::ser::Error) -> Self {
        Error::URLSerialization(e)
    }
}
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IO(e)
    }
}
impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::JSON(e)
    }
}
impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}
impl From<reqwest::header::InvalidHeaderValue> for Error {
    fn from(e: reqwest::header::InvalidHeaderValue) -> Self {
        Error::InvalidHeaderValue(e)
    }
}
impl From<url::ParseError> for Error {
    fn from(e: url::ParseError) -> Self {
        Error::URLParse(e)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IO(e) => write!(f, "IO error: {}", e),
            Error::Reqwest(e) => write!(f, "Reqwest error: {}", e),
            Error::URLParse(e) => write!(f, "URL parse error: {}", e),
            Error::Other(e) => write!(f, "Other error: {}", e.as_str()),
            Error::InvalidHeaderValue(e) => write!(f, "Invalid header value error: {}", e),
            Error::JSON(e) => write!(f, "JSON error: {}", e),
            Error::URLSerialization(e) => write!(f, "URL serialization error: {}", e),
            Error::ChannelError => write!(f, "MPSC channel error"),
        }
    }
}
impl std::error::Error for Error {}
