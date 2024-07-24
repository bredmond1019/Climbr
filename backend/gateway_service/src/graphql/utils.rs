use serde_json::Value;

use super::schema::Context;

pub async fn get_graphql_response(query_string: &str, context: &Context) -> Value {
    let url = format!("{}/graphql", context.user_service_url);
    let response = context
        .client
        .post(&url)
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
