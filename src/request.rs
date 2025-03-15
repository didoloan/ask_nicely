pub trait Request<'a>
where
    Self: serde::Serialize + 'a,
    Self::Response: serde::de::DeserializeOwned,
    Self::ReqObj: serde::Serialize + 'a,
{
    // concatenate to get with path params
    type Response;
    type ReqObj;
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const AUTHED: bool;
    fn get_path(&'a self) -> std::borrow::Cow<'a, str>;
    fn get_data(&'a self) -> crate::request_data::RequestData<'a, Self::ReqObj>;

    fn to_api_call(&'a self) -> crate::api_call::ApiCall<'a, Self::ReqObj, Self::Response> {
        crate::api_call::ApiCall::new(self.get_path(), Self::METHOD, self.get_data(), Self::AUTHED)
    }
}
