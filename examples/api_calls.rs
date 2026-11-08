//! Calls Openapi® APIs with a Bearer access token.

use openapi_sdk::Client;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
struct Query {
    country_code: String,
}

#[derive(Serialize)]
struct Payload {
    limit: u64,
    query: Query,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("<your_access_token>")?;

    let mut params = HashMap::new();
    params.insert("denominazione", "altravia");
    params.insert("provincia", "RM");
    params.insert("codice_ateco", "6201");

    let result = client
        .request::<serde_json::Value>(
            "GET",
            "https://test.imprese.openapi.it/advance",
            None,
            Some(params),
        )
        .await?;

    println!("API Response: {result}");

    let query = Query {
        country_code: "IT".to_string(),
    };
    let payload = Payload { limit: 10, query };

    let result = client
        .request(
            "POST",
            "https://test.postontarget.com/fields/country",
            Some(&payload),
            None,
        )
        .await?;

    println!("POST API Response: {result}");

    Ok(())
}
