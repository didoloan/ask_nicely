use reqwest::header::{HeaderName, HeaderValue};
use std::str::FromStr;

#[derive(Debug)]
pub enum Authentication<'a> {
    Basic(&'a str, Option<&'a str>),
    Digest(&'a str, Option<&'a str>),
    Bearer(&'a str),
    ApiKeys(Vec<(&'a str, &'a str)>),
    None,
}

impl Clone for Authentication<'_> {
    fn clone(&self) -> Self {
        match self {
            Self::Basic(arg0, arg1) => Self::Basic(arg0, *arg1),
            Self::Digest(arg0, arg1) => Self::Digest(arg0, *arg1),
            Self::Bearer(arg0) => Self::Bearer(arg0),
            Self::ApiKeys(arg0) => Self::ApiKeys(arg0.clone()),
            Self::None => Self::None,
        }
    }
}

impl<'a> crate::prelude::MutateReq for (&'a Authentication<'a>, bool) {
    fn mutate_req(&self, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        match &self {
            (Authentication::Basic(uname, pass), true) => {
                request.basic_auth::<&str, &str>(*uname, *pass)
            }
            (Authentication::Bearer(key), true) => request.bearer_auth(*key),
            (Authentication::ApiKeys(items), true) => {
                let header_mapped = items.iter().map(|(x, y)| {
                    (
                        HeaderName::from_str(x).unwrap(),
                        HeaderValue::from_str(y).unwrap(),
                    )
                });

                let map = reqwest::header::HeaderMap::from_iter(header_mapped);

                request.headers(map)
            }
            (_, _) => request,
        }
    }
}
