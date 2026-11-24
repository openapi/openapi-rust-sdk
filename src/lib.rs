use base64;
use http::header::{AUTHORIZATION, CONTENT_TYPE};
use http::HeaderMap;
use reqwest::Client as reqwest_client;
use reqwest::{Method, Response};

use serde::Serialize;
use std::collections::HashMap;
use std::error::Error as StdError;
use std::fmt;
use std::str::FromStr;

// Base URL for production OAuth endpoint
const OAUTH_BASE_URL: &str = "https://oauth.openapi.it";
// Base URL for test OAuth endpoint
const TEST_OAUTH_BASE_URL: &str = "https://test.oauth.openapi.it";

/// Result type returned by the Openapi SDK.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors produced by the Openapi SDK.
///
/// This type wraps lower-level failures so applications can add a single
/// `From<openapi_sdk::Error>` implementation to their own error enum and keep
/// using the `?` operator in existing `Result` pipelines.
#[derive(Debug)]
pub enum Error {
    /// Error returned by the HTTP client.
    Http(reqwest::Error),
    /// Error caused by an invalid authorization or content type header value.
    InvalidHeaderValue(http::header::InvalidHeaderValue),
    /// Error caused by an invalid HTTP method string.
    InvalidMethod(http::method::InvalidMethod),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Http(error) => write!(f, "HTTP request failed: {error}"),
            Self::InvalidHeaderValue(error) => write!(f, "invalid HTTP header value: {error}"),
            Self::InvalidMethod(error) => write!(f, "invalid HTTP method: {error}"),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Http(error) => Some(error),
            Self::InvalidHeaderValue(error) => Some(error),
            Self::InvalidMethod(error) => Some(error),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self::Http(error)
    }
}

impl From<http::header::InvalidHeaderValue> for Error {
    fn from(error: http::header::InvalidHeaderValue) -> Self {
        Self::InvalidHeaderValue(error)
    }
}

impl From<http::method::InvalidMethod> for Error {
    fn from(error: http::method::InvalidMethod) -> Self {
        Self::InvalidMethod(error)
    }
}

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
    pub fn new(username: &str, apikey: &str, test: bool) -> Result<OauthClient> {
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

        headers.insert(AUTHORIZATION, auth_header.parse()?);
        headers.insert(CONTENT_TYPE, "application/json".parse()?);

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
    pub async fn get_scopes(&self, limit: bool) -> Result<String> {
        let params = [("limit", limit as u8)];
        let url = format!("{}/scopes", self.url);
        let response: Response = self.client.get(url).query(&params).send().await?;
        let response = response.error_for_status()?;
        let json_str: String = response.text().await?;
        Ok(json_str)
    }

    /// Creates a new access token with specified scopes and time-to-live
    ///
    /// # Arguments
    /// * `scopes` - List of permission scopes for the token
    /// * `ttl` - Token lifetime in seconds
    pub async fn create_token(&self, scopes: Vec<&'static str>, ttl: u64) -> Result<String> {
        // Request body structure for token creation
        #[derive(Serialize)]
        struct Body {
            scopes: Vec<&'static str>,
            ttl: u64,
        }

        let body = Body { scopes, ttl };
        let url = format!("{}/token", self.url);
        let response: Response = self.client.post(url).json(&body).send().await?;
        let response = response.error_for_status()?;
        let json_str: String = response.text().await?;
        Ok(json_str)
    }

    /// Retrieves existing tokens filtered by scope
    ///
    /// # Arguments
    /// * `scope` - The scope to filter tokens by
    pub async fn get_tokens(&self, scope: &'static str) -> Result<String> {
        let params = [("scope", scope)];
        let url = format!("{}/token", self.url);
        let response: Response = self.client.get(url).query(&params).send().await?;
        let response = response.error_for_status()?;
        let json_str: String = response.text().await?;
        Ok(json_str)
    }

    /// Deletes a token by its ID
    ///
    /// # Arguments
    /// * `id` - The unique identifier of the token to delete
    pub async fn delete_token(&self, id: String) -> Result<String> {
        let url = format!("{}/token/{}", self.url, id);
        let response: Response = self.client.delete(url).send().await?;
        let response = response.error_for_status()?;
        let json_str: String = response.text().await?;
        Ok(json_str)
    }

    /// Retrieves API usage counters for a specific period and date
    ///
    /// # Arguments
    /// * `period` - The time period (e.g., "day", "month")
    /// * `date` - The date in appropriate format
    pub async fn get_counters(&self, period: &'static str, date: &'static str) -> Result<String> {
        let url = format!("{}/counters/{}/{}", self.url, period, date);
        let response: Response = self.client.get(url).send().await?;
        let response = response.error_for_status()?;
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
    pub fn new(token: String) -> Result<Client> {
        let auth_header = format!("Bearer {token}");
        let mut headers = HeaderMap::new();

        headers.insert(AUTHORIZATION, auth_header.parse()?);
        headers.insert(CONTENT_TYPE, "application/json".parse()?);

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
    ) -> Result<String>
    where
        T: Serialize,
    {
        let url = format!("{}", url);

        let mut request = self.client.request(Method::from_str(method)?, url);

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
        let response = response.error_for_status()?;
        let json_str: String = response.text().await?;
        Ok(json_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error as StdError;
    use std::fmt;

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

    #[test]
    fn sdk_error_implements_std_error_traits() {
        fn assert_error_traits<T: StdError + Send + Sync + 'static>() {}

        assert_error_traits::<Error>();
    }

    #[test]
    fn invalid_auth_header_returns_error() {
        let error = match Client::new("invalid\ntoken".to_string()) {
            Ok(_) => panic!("client creation unexpectedly succeeded"),
            Err(error) => error,
        };

        assert!(matches!(error, Error::InvalidHeaderValue(_)));
    }

    #[tokio::test]
    async fn invalid_http_method_returns_error() {
        let client = Client::new("test_token".to_string()).unwrap();
        let error = client
            .request::<serde_json::Value>("BAD METHOD", "https://example.com", None, None)
            .await
            .unwrap_err();

        assert!(matches!(error, Error::InvalidMethod(_)));
    }

    #[test]
    fn sdk_error_converts_into_custom_error_for_question_mark_pipeline() {
        #[derive(Debug)]
        enum MyError {
            Openapi(Error),
        }

        impl fmt::Display for MyError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    Self::Openapi(error) => write!(f, "{error}"),
                }
            }
        }

        impl StdError for MyError {
            fn source(&self) -> Option<&(dyn StdError + 'static)> {
                match self {
                    Self::Openapi(error) => Some(error),
                }
            }
        }

        impl From<Error> for MyError {
            fn from(error: Error) -> Self {
                Self::Openapi(error)
            }
        }

        fn create_client() -> std::result::Result<Client, MyError> {
            let client = Client::new("invalid\ntoken".to_string())?;
            Ok(client)
        }

        assert!(matches!(
            create_client(),
            Err(MyError::Openapi(Error::InvalidHeaderValue(_)))
        ));
    }

    // TODO: Add integration tests for API endpoints with mock server
}
