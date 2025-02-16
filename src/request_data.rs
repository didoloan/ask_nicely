use crate::prelude::MutateReq;
use reqwest::header::CONTENT_TYPE;
use serde::Serialize;

#[derive(Debug)]
pub enum RequestData<'a, T> {
    Query(&'a T),
    Json(&'a T),
    Xml(&'a T),
    Form(&'a T),
    Binary(bytes::Bytes),
    // PayloadBinary(
    //     &'a mut (dyn futures::Stream<Item = Result<bytes::Bytes, Box<dyn std::error::Error + Sync + Send>>>
    //                  + Send
    //                  + Unpin + 'static),
    // ),
    None,
}

impl<T: Serialize> MutateReq for RequestData<'_, T> {
    fn mutate_req(&self, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        match self {
            RequestData::Query(obj) => request.query(*obj),
            RequestData::Json(obj) => request.json(obj),
            RequestData::Xml(obj) => request
                .body(serde_xml_rs::to_string(*obj).unwrap())
                .header(CONTENT_TYPE, "application/xml"),
            RequestData::Form(obj) => request.form(*obj),
            RequestData::Binary(pload) => request
                .body::<Vec<u8>>(pload.to_vec())
                .header(CONTENT_TYPE, "application/octet-stream"),
            RequestData::None => request,
        }
    }
}
