#[derive(Debug)]
pub struct ApiCall<'a, T, R>
where
    T: serde::Serialize + 'a,
    R: serde::de::DeserializeOwned,
{
    pub path: std::borrow::Cow<'a, str>,
    pub method: reqwest::Method,
    pub request: crate::request_data::RequestData<'a, T>,
    pub authed: bool,
    _phantom: std::marker::PhantomData<R>,
}

impl<'a, T, R> ApiCall<'a, T, R>
where
    T: serde::Serialize + 'a,
    R: serde::de::DeserializeOwned,
{
    pub fn new(
        path: std::borrow::Cow<'a, str>,
        method: reqwest::Method,
        request: crate::request_data::RequestData<'a, T>,
        authed: bool,
    ) -> Self {
        Self {
            path,
            method,
            request,
            authed,
            _phantom: std::marker::PhantomData,
        }
    }
}
