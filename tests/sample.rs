use ask_nicely::prelude::*;
use serde::{Deserialize, Serialize};
use tokio;

#[nicely_macros::gen_client("https://api.weatherclient.com")]
struct WeatherClient;

#[derive(Debug, serde::Serialize, nicely_macros::Request)]
#[request(path="/weather/{country}/{state}", authed=false, response=Weather, method="get", get_data=None)]
pub struct WeatherReq<'a> {
    pub country: &'a str,
    pub state: &'a str,
}

#[derive(Debug, serde::Deserialize)]
pub struct Weather {
    temp: i8,
    desc: String,
}

#[tokio::test]
async fn main() {
    let weather_client = WeatherClient::builder()
        .set_auth(Authentication::Bearer("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c"))
        .build()
        .unwrap();

    let weather_req = WeatherReq {
        country: "USA",
        state: "Texas",
    };

    let path = weather_req.get_path();

    let response = weather_client.request(&weather_req).await;

    println!("{}", path);
}
