use reqwest::{Client, Response};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, AUTHORIZATION};
use std::time::Duration;

pub struct RocketAPI {
    base_url: String,
    token: String,
    max_timeout: Duration,
}

impl RocketAPI {
    pub fn new(token: String, max_timeout: Duration) -> Self {
        RocketAPI {
            base_url: "https://v1.rocketapi.io/".to_string(),
            token,
            max_timeout: max_timeout,
        }
    }
    
    pub async fn request(&self, method: &str, data: serde_json::Value) -> Result<serde_json::Value, reqwest::Error> {
        let client = Client::builder()
            .timeout(self.max_timeout)
            .build()?;

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Token {}", self.token)).unwrap());

        let url = format!("{}{}", self.base_url, method);
        let response: Response = client.post(&url)
            .headers(headers)
            .json(&data)
            .send()
            .await?;

        let json_response: serde_json::Value = response.json().await?;
        Ok(json_response)
    }
}
