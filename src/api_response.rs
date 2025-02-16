use reqwest::StatusCode;
use serde::Deserialize;

#[derive(Debug)]
pub enum ApiResponse<T> {
    Success(T),
    Failure(ApiError),
}

#[derive(Debug)]
pub struct ApiError {
    pub status: StatusCode,
    pub body: String,
}

impl ApiError {
    pub fn new(status: StatusCode, body: String) -> Self {
        Self { status, body }
    }

    pub fn get_typed<E: for<'de> Deserialize<'de>>(&self) -> Result<E, impl serde::de::Error> {
        serde_json::from_str(self.body.as_str())
    }
}
