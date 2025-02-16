use crate::authentication::Authentication;
use crate::client::ApiClient;

pub struct ApiClientBuilder<'a> {
    inner: reqwest::ClientBuilder,
    base_url: &'a str,
    auth: Authentication<'a>,
}

impl<'a> ApiClientBuilder<'a> {
    pub fn new(base_url: &'a str) -> Self {
        Self {
            inner: reqwest::Client::builder(),
            auth: Authentication::None,
            base_url,
        }
    }

    pub fn set_auth(mut self, auth: Authentication<'a>) -> Self {
        self.auth = auth;
        self
    }

    pub fn set_timeout(mut self, timeout: std::time::Duration) -> Self {
        self.inner = self.inner.timeout(timeout);
        self
    }

    pub fn redirect_policy(mut self, policy: reqwest::redirect::Policy) -> Self {
        self.inner = self.inner.redirect(policy);
        self
    }

    pub fn set_redirect_policy(mut self, policy: reqwest::redirect::Policy) -> Self {
        self.inner = self.inner.redirect(policy);
        self
    }

    pub fn build(self) -> reqwest::Result<ApiClient<'a>> {
        Ok(ApiClient {
            base_url: self.base_url,
            auth: self.auth,
            client: self.inner.build()?,
        })
    }
}
