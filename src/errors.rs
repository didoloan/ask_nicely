use std::fmt::{self, Formatter};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    Request(reqwest::Error),
    UrlParseErr(url::ParseError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::Request(error) => error.fmt(f),
            Error::UrlParseErr(parse_error) => parse_error.fmt(f),
        }
    }
}

impl From<url::ParseError> for Error {
    fn from(value: url::ParseError) -> Self {
        Self::UrlParseErr(value)
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::Request(value)
    }
}

// impl From<ApiError> for Error {
//     fn from(value: ApiError) -> Self {
//         Self::ErrorFromApi(value)
//     }
// }
