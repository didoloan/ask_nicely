pub mod api_call;
pub mod api_response;
pub mod authentication;
pub mod client;
pub mod client_builder;
pub mod errors;
pub mod mutate_req;
pub mod request;
pub mod request_data;

pub mod prelude {
    use super::*;
    pub use crate::request;
    pub use nicely_macros;
    pub use nicely_macros::{gen_client, Request};
    pub use api_call::ApiCall;
    pub use api_response::ApiResponse;
    pub use authentication::Authentication;
    pub use client::{ApiClient, CanApiClient};
    pub use client_builder::ApiClientBuilder;
    pub use errors::Error;
    pub use mutate_req::MutateReq;
    pub use request::Request;
    pub use request_data::RequestData;
}
