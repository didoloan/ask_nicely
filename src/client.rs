use crate::api_response::ApiError;
use crate::authentication::Authentication;
use crate::mutate_req::MutateReq;
use std::str::FromStr;

pub struct ApiClient<'a> {
    pub base_url: &'a str,
    pub auth: Authentication<'a>,
    pub client: reqwest::Client,
}

pub trait CanApiClient<'a> {
    fn get_auth(&'a self) -> &'a Authentication<'a>;
    fn get_base_url(&self) -> String;
    fn get_client(&'a self) -> &'a reqwest::Client;

    fn request<'q, REQ>(
        &'a self,
        call: &'a REQ,
    ) -> impl futures::Future<Output = Result<Result<REQ::Response, ApiError>, crate::errors::Error>>
    where
        REQ: crate::request::Request<'a> + 'a,
        REQ::ReqObj: serde::Serialize + 'a,
        REQ::Response: serde::de::DeserializeOwned + serde::Serialize,
    {
        async {
            let api_call = call.to_api_call();

            let url = reqwest::Url::from_str(self.get_base_url().trim())?
            .join(api_call.path.as_ref())?;

            let mut request = self.get_client().request(api_call.method.clone(), url);

            request = api_call.request.mutate_req(request);

            request = (self.get_auth(), api_call.authed).mutate_req(request);

            let response = request.send().await?;

            let status = response.status();

            if status.is_success() {
                return Ok(Ok(response.json::<REQ::Response>().await?));
            }

            Ok(Err(ApiError::new(status, response.text().await?)))
        }
    }
}

impl<'a> CanApiClient<'a> for ApiClient<'a> {
    fn get_auth(&'a self) -> &'a Authentication<'a> {
        &self.auth
    }

    fn get_base_url(&self) -> String {
        self.base_url.to_string()
    }

    fn get_client(&'a self) -> &'a reqwest::Client {
        &self.client
    }
}

impl<'a> ApiClient<'a> {
    pub fn builder(base_url: &'a str) -> crate::client_builder::ApiClientBuilder<'a> {
        crate::client_builder::ApiClientBuilder::new(base_url)
    }

    pub fn new(base_url: &'a str, auth: Authentication<'a>) -> Self {
        let client = reqwest::Client::new();

        Self {
            base_url,
            auth,
            client,
        }
    }

    // pub fn request<'q, REQ>(
    //     &'a self,
    //     call: &'a REQ,
    // ) -> impl futures::Future<Output = Result<Result<REQ::Response, ApiError>, crate::errors::Error>>
    // where
    //     REQ: crate::request::Request<'a> + 'a,
    //     REQ::ReqObj: serde::Serialize + 'a,
    //     REQ::Response: serde::de::DeserializeOwned + serde::Serialize,
    // {
    //     async {
    //         let api_call = call.to_api_call();

    //         let url = reqwest::Url::from_str(self.base_url.trim())?
    //             .join(api_call.path.as_ref())?;

    //         let mut request = self.get_client().request(api_call.method.clone(), url);

    //         request = api_call.request.mutate_req(request);

    //         request = (self.get_auth(), api_call.authed).mutate_req(request);

    //         let response = request.send().await?;

    //         let status = response.status();

    //         if status.is_success() {
    //             return Ok(Ok(response.json::<REQ::Response>().await?));
    //         }

    //         Ok(Err(ApiError::new(status, response.text().await?)))
    //     }
    // }
}
