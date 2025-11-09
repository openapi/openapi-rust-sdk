use base64;
use http::header::{AUTHORIZATION, CONTENT_TYPE};
use http::HeaderMap;
use reqwest::Client as reqwest_client;
use reqwest::{Method, Response};

use serde::Serialize;
use std::collections::HashMap;
use std::str::FromStr;

const OAUTH_BASE_URL: &str = "https://oauth.openapi.it";
const TEST_OAUTH_BASE_URL: &str = "https://test.oauth.openapi.it";

pub struct OauthClient {
    client: reqwest_client,
    url: &'static str,
}

impl OauthClient {
    pub fn new(username: &str, apikey: &str, test: bool) -> Result<OauthClient, reqwest::Error> {
        let url = if test {
            TEST_OAUTH_BASE_URL
        } else {
            OAUTH_BASE_URL
        };
        let encoded = base64::encode(format!("{username}:{apikey}"));
        let auth_header = format!("Basic {encoded}");
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, auth_header.parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;
        Ok(OauthClient { client, url })
    }

    pub async fn get_scopes(&self, limit: bool) -> Result<String, reqwest::Error> {
        let params = [("limit", limit as u8)];
        let url = format!("{}/scopes", self.url);
        let response: Response = self.client.get(url).query(&params).send().await?;
        let json_str: String = response.text().await?;
        Ok(json_str)
    }

    pub async fn create_token(
        &self,
        scopes: Vec<&'static str>,
        ttl: u64,
    ) -> Result<String, reqwest::Error> {
        #[derive(Serialize)]
        struct Body {
            scopes: Vec<&'static str>,
            ttl: u64,
        }

        let body = Body { scopes, ttl };
        let url = format!("{}/token", self.url);
        let response: Response = self.client.post(url).json(&body).send().await?;
        let json_str: String = response.text().await?;
        Ok(json_str)
    }

    pub async fn get_tokens(&self, scope: &'static str) -> Result<String, reqwest::Error> {
        let params = [("scope", scope)];
        let url = format!("{}/token", self.url);
        let response: Response = self.client.get(url).query(&params).send().await?;
        let json_str: String = response.text().await?;
        Ok(json_str)
    }

    pub async fn delete_token(&self, id: String) -> Result<String, reqwest::Error> {
        let url = format!("{}/token/{}", self.url, id);
        let response: Response = self.client.delete(url).send().await?;
        let json_str: String = response.text().await?;
        Ok(json_str)
    }

    pub async fn get_counters(
        &self,
        period: &'static str,
        date: &'static str,
    ) -> Result<String, reqwest::Error> {
        let url = format!("{}/counters/{}/{}", self.url, period, date);
        let response: Response = self.client.get(url).send().await?;
        let json_str: String = response.text().await?;
        Ok(json_str)
    }
}

pub struct Client {
    client: reqwest_client,
}

impl Client {
    pub fn new(token: String) -> Result<Client, reqwest::Error> {
        let auth_header = format!("Bearer {token}");
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, auth_header.parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;
        Ok(Client { client })
    }

    pub async fn request<T>(
        &self,
        method: &str,
        url: &str,
        payload: Option<&T>,
        params: Option<HashMap<&str, &str>>,
    ) -> Result<String, reqwest::Error>
    where
        T: Serialize,
    {
        let url = format!("{}", url);
        let mut request = self.client.request(Method::from_str(method).unwrap(), url);

        if let Some(payload) = payload {
            request = request.json(payload);
        }

        if let Some(params) = params {
            request = request.query(&params);
        }

        let response: Response = request.send().await?;
        let json_str: String = response.text().await?;
        Ok(json_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oauth_client_creation() {
        let client = OauthClient::new("test_user", "test_key", true);
        assert!(client.is_ok());
    }

    #[test]
    fn test_api_client_creation() {
        let client = Client::new("test_token".to_string());
        assert!(client.is_ok());
    }
}