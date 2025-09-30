use openapi_client::OauthClient;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct TokenResponse {
    token: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let oauth_client = OauthClient::new("<your_username>", "<your_apikey>", true)?;

    let scopes = vec![
        "GET:test.imprese.openapi.it/advance",
        "POST:test.postontarget.com/fields/country",
    ];
    let ttl = 3600;
    let result = oauth_client.create_token(scopes, ttl).await?;

    let resp: TokenResponse = serde_json::from_str(&result)?;
    println!("Generated token: {}", resp.token);

    println!("Token created successfully!");
    Ok(())
}