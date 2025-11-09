use base64;
use http::header::{AUTHORIZATION, CONTENT_TYPE};
use http::HeaderMap;
use reqwest::Client as reqwest_client;
use reqwest::{Method, Response};

use serde::Serialize;
use std::collections::HashMap;
use std::str::FromStr;

// Base URL for production OAuth endpoint
const OAUTH_BASE_URL: &str = "https://oauth.openapi.it";
// Base URL for test OAuth endpoint
const TEST_OAUTH_BASE_URL: &str = "https://test.oauth.openapi.it";

/// OAuth client for OpenAPI authentication and token management
pub struct OauthClient {
    client: reqwest_client,
    url: &'static str,
}

impl OauthClient {
    /// Creates a new OAuth client with Basic authentication
    ///
    /// # Arguments
    /// * `username` - The API username
    /// * `apikey` - The API key for authentication
    /// * `test` - If true, uses test environment; otherwise production
    pub fn new(username: &str, apikey: &str, test: bool) -> Result<OauthClient, reqwest::Error> {
        // Select appropriate base URL based on environment
        let url = if test {
            TEST_OAUTH_BASE_URL
        } else {
            OAUTH_BASE_URL
        };

        // Encode credentials for Basic auth
        let encoded = base64::encode(format!("{username}:{apikey}"));
        let auth_header = format!("Basic {encoded}");
        let mut headers = HeaderMap::new();

        // TODO: Replace unwrap() with graceful error message for invalid header values
        headers.insert(AUTHORIZATION, auth_header.parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        // Build HTTP client with default headers
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;
        Ok(OauthClient { client, url })
    }

    /// Retrieves available OAuth scopes
    ///
    /// # Arguments
    /// * `limit` - If true, returns limited scope information
    pub async fn get_scopes(&self, limit: bool) -> Result<String, reqwest::Error> {
        let params = [("limit", limit as u8)];
        let url = format!("{}/scopes", self.url);
        let response: Response = self.client.get(url).query(&params).send().await?;
        // TODO: Add error handling for non-2xx status codes with descriptive messages
        let json_str: String = response.text().await?;
        Ok(json_str)
    }

    /// Creates a new access token with specified scopes and time-to-live
    ///
    /// # Arguments
    /// * `scopes` - List of permission scopes for the token
    /// * `ttl` - Token lifetime in seconds
    pub async fn create_token(
        &self,
        scopes: Vec<&'static str>,
        ttl: u64,
    ) -> Result<String, reqwest::Error> {
        // Request body structure for token creation
        #[derive(Serialize)]
        struct Body {
            scopes: Vec<&'static str>,
            ttl: u64,
        }

        let body = Body { scopes, ttl };
        let url = format!("{}/token", self.url);
        let response: Response = self.client.post(url).json(&body).send().await?;
        // TODO: Check response status and provide meaningful error for failed token creation
        let json_str: String = response.text().await?;
        Ok(json_str)
    }

    /// Retrieves existing tokens filtered by scope
    ///
    /// # Arguments
    /// * `scope` - The scope to filter tokens by
    pub async fn get_tokens(&self, scope: &'static str) -> Result<String, reqwest::Error> {
        let params = [("scope", scope)];
        let url = format!("{}/token", self.url);
        let response: Response = self.client.get(url).query(&params).send().await?;
        // TODO: Add error handling for invalid scope or empty results
        let json_str: String = response.text().await?;
        Ok(json_str)
    }

    /// Deletes a token by its ID
    ///
    /// # Arguments
    /// * `id` - The unique identifier of the token to delete
    pub async fn delete_token(&self, id: String) -> Result<String, reqwest::Error> {
        let url = format!("{}/token/{}", self.url, id);
        let response: Response = self.client.delete(url).send().await?;
        // TODO: Provide clear error message when token not found or already deleted
        let json_str: String = response.text().await?;
        Ok(json_str)
    }

    /// Retrieves API usage counters for a specific period and date
    ///
    /// # Arguments
    /// * `period` - The time period (e.g., "day", "month")
    /// * `date` - The date in appropriate format
    pub async fn get_counters(
        &self,
        period: &'static str,
        date: &'static str,
    ) -> Result<String, reqwest::Error> {
        let url = format!("{}/counters/{}/{}", self.url, period, date);
        let response: Response = self.client.get(url).send().await?;
        // TODO: Validate period and date format, provide descriptive error for invalid values
        let json_str: String = response.text().await?;
        Ok(json_str)
    }
}

/// Generic API client with Bearer token authentication
pub struct Client {
    client: reqwest_client,
}

impl Client {
    /// Creates a new API client with Bearer token authentication
    ///
    /// # Arguments
    /// * `token` - The Bearer token for API authentication
    pub fn new(token: String) -> Result<Client, reqwest::Error> {
        let auth_header = format!("Bearer {token}");
        let mut headers = HeaderMap::new();

        // TODO: Replace unwrap() with graceful error message for invalid token format
        headers.insert(AUTHORIZATION, auth_header.parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        // Build HTTP client with Bearer auth headers
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;
        Ok(Client { client })
    }

    /// Makes an HTTP request to the specified URL
    ///
    /// # Arguments
    /// * `method` - HTTP method as string (e.g., "GET", "POST")
    /// * `url` - The full URL to request
    /// * `payload` - Optional JSON payload for the request body
    /// * `params` - Optional query parameters
    ///
    /// # Returns
    /// The response body as a JSON string
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

        // TODO: Replace unwrap() with proper error handling for invalid HTTP methods
        // Consider returning a descriptive error like "Invalid HTTP method: {method}"
        let mut request = self.client.request(Method::from_str(method).unwrap(), url);

        // Attach JSON payload if provided
        if let Some(payload) = payload {
            request = request.json(payload);
        }

        // Attach query parameters if provided
        if let Some(params) = params {
            request = request.query(&params);
        }

        // Execute the request
        let response: Response = request.send().await?;
        // TODO: Check HTTP status code and provide meaningful error messages for different status codes
        // (e.g., 400 Bad Request, 401 Unauthorized, 404 Not Found, 500 Server Error)
        let json_str: String = response.text().await?;
        Ok(json_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests successful creation of OAuth client with test credentials
    #[test]
    fn test_oauth_client_creation() {
        let client = OauthClient::new("test_user", "test_key", true);
        assert!(client.is_ok());
    }

    /// Tests successful creation of API client with Bearer token
    #[test]
    fn test_api_client_creation() {
        let client = Client::new("test_token".to_string());
        assert!(client.is_ok());
    }

    // TODO: Add test for invalid credentials and verify error messages are descriptive
    // TODO: Add test for invalid HTTP method in request() and verify error handling
    // TODO: Add integration tests for API endpoints with mock server
}