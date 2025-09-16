<div align="center">
  <a href="https://openapi.com/">
    <img alt="Algolia for PHP" src=".github/assets/repo-header-a3.png" >
  </a>

<h1>openapi® client for Rust</h1>
<h4>The perfect starting point to integrate <a href="https://openapi.com/">openapi®</a> within your Rust project</h4>
</div>

This client is used to interact with the API found at [openapi.it](https://openapi.it/)

## Pre-requisites

Before using the OpenApi IT Rust Client, you will need an account at [openapi.it](https://openapi.it/) and an API key to the sandbox and/or production environment

## Installation

You can add the OpenApi IT Rust Client to your project with the following command:

```bash
cargo add openapiit-cli-rust
```
    
## Usage

```rust
use std::collections::HashMap;

use openapi_client::{Client, OauthClient};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct TokenResponse {
    token: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the oauthclient
    let oauth_client =
        OauthClient::new("<your_username>", "<your_apikey>", true).unwrap();

    // Create a token for a list of scopes
    let scopes = vec![
        "GET:test.imprese.openapi.it/advance",
        "POST:test.postontarget.com/fields/country",
    ];
    let ttl = 3600;
    let result = oauth_client.create_token(scopes, ttl).await?;

    // The string response can be parsed into a custom object
    let resp: TokenResponse = serde_json::from_str(&result)?;
    let token = resp.token;

    // Initialize the client
    let client = Client::new(token.clone()).unwrap();

    // Make a request with Params
    let mut params = HashMap::new();
    params.insert("denominazione", "altravia");
    params.insert("provincia", "RM");
    params.insert("codice_ateco", "6201");

    let _result = client
        .request::<serde_json::Value>(
            "GET",
            "https://test.imprese.openapi.it/advance",
            None,
            Some(params),
        )
        .await?;

    // Make a request with a payload, a nested json
    #[derive(Serialize)]
    struct Query {
        country_code: String,
    }

    #[derive(Serialize)]
    struct Payload {
        limit: u64,
        query: Query,
    }

    let query = Query {
        country_code: "IT".to_string(),
    };
    let payload = Payload { limit: 0, query };

    let _result = client
        .request(
            "POST",
            "https://test.postontarget.com/fields/country",
            Some(&payload),
            None,
        )
        .await?;

    // Delete the token
    let _result = oauth_client.delete_token(token).await?;

    Ok(())
}
```

## Contributing

Contributions are always welcome!

See `contributing.md` for ways to get started.

Please adhere to this project's `code of conduct`.


## License

[MIT](https://choosealicense.com/licenses/mit/)


## Authors

- [@maiku1008](https://www.github.com/maiku1008)
- [@openapi-it](https://github.com/openapi-it)
