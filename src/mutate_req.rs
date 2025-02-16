use reqwest::header::{HeaderMap, HeaderValue};

pub trait MutateReq {
    fn mutate_req(&self, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder;
}

impl MutateReq for Option<HeaderMap<HeaderValue>> {
    fn mutate_req(&self, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(headers) = self {
            return request.headers(headers.clone());
        }

        request
    }
}
