use ask_nicely::prelude::*;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::hash_set::Drain;
use std::collections::HashMap;
use tokio;

#[nicely_macros::gen_client("https://api.weatherclient.com")]
struct WeatherClient;

#[derive(Debug, serde::Serialize, nicely_macros::Request)]
#[request(path="/weather/{}/{}", authed, response=Weather, method="get", get_data=None)]
pub struct WeatherReq<'a> {
    #[request(path_var=0)]
    pub country: &'a str,
    #[request(path_var=1)]
    pub state: &'a str,
}

#[derive(Debug, serde::Deserialize)]
pub struct Weather {
    temp: i8,
    desc: String
}

#[tokio::test]
async fn main() {

    let drink_client = WeatherClient::builder().set_auth(Authentication::None).build().unwrap();

    let req = WeatherReq {
        country: "USA",
        state: "Texas"
    };

    let response = drink_client.request(&req).await.unwrap();

    println!("{:?}", response);
}
