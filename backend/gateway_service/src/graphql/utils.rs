use reqwest::{Client, Error};
use serde_json::Value;

pub async fn graphql_request(
    query_string: &str,
    client: &Client,
    service_url: &str,
) -> Result<Value, Error> {
    let url = format!("{}/graphql", service_url);
    let response = client
        .post(&url)
        .json(&serde_json::json!({
            "query": query_string
        }))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    Ok(response)
}

pub async fn graphql_request_with_token(
    query_string: &str,
    client: &Client,
    service_url: &str,
    token: String,
) -> Value {
    let url = format!("{}/graphql", service_url);
    let response = client
        .post(&url)
        .header("Authorization", token)
        .json(&serde_json::json!({
            "query": query_string
        }))
        .send()
        .await
        .expect("Error sending request")
        .json::<serde_json::Value>()
        .await
        .expect("Error parsing response");

    response
}
