use reqwest::Client;

pub async fn graphql_request(
    query_string: &str,
    client: &Client,
    service_url: &str,
) -> Result<serde_json::Value, reqwest::Error> {
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
