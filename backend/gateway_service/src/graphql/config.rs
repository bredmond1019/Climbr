use log::error;
use std::env;
use std::process::{Command, Output};

fn publish_schema(
    subgraph_name: String,
    schema_path: String,
    routing_url: String,
    graph_id: String,
) -> Output {
    // rover subgraph publish <YOUR_APOLLO_GRAPH_ID>
    // --name <SUBGRAPH_NAME>
    // --schema ./path/to/your/schema.graphql
    // --routing-url <YOUR_SUBGRAPH_URL>

    Command::new("rover")
        .arg("subgraph")
        .arg("publish")
        .arg(graph_id)
        .arg("--name")
        .arg(subgraph_name)
        .arg("--schema")
        .arg(schema_path)
        .arg("--routing-url")
        .arg(routing_url)
        .output()
        .expect("Failed to publish subgraph schema")
}

pub fn publish_user_service_schema() {
    let subgraph_name = env::var("USER_SUBGRAPH_NAME").expect("USER_SUBGRAPH_NAME must be set");
    let schema_path = env::var("USER_SCHEMA_PATH").expect("USER_SCHEMA_PATH must be set");
    let routing_url = env::var("USER_ROUTING_URL").expect("USER_ROUTING_URL must be set");
    let graph_id = env::var("APOLLO_GRAPH_ID").expect("APOLLO_GRAPH_ID must be set");

    let output = publish_schema(subgraph_name, schema_path, routing_url, graph_id);

    if !output.status.success() {
        error!("Failed to publish user service schema: {:?}", output);
    }
}

pub fn publish_schedule_service_schema() {
    let subgraph_name =
        env::var("SCHEDULE_SUBGRAPH_NAME").expect("SCHEDULE_SUBGRAPH_NAME must be set");
    let schema_path = env::var("SCHEDULE_SCHEMA_PATH").expect("SCHEDULE_SCHEMA_PATH must be set");
    let routing_url = env::var("SCHEDULE_ROUTING_URL").expect("SCHEDULE_ROUTING_URL must be set");
    let graph_id = env::var("APOLLO_GRAPH_ID").expect("APOLLO_GRAPH_ID must be set");

    let output = publish_schema(subgraph_name, schema_path, routing_url, graph_id);

    if !output.status.success() {
        error!("Failed to publish schedule service schema: {:?}", output);
    }
}

pub fn config_apollo_router() -> Output {
    let router_path = "./router"; // Adjust the path as necessary
    let config_path = "router.yaml"; // Adjust the path as necessary

    Command::new(router_path)
        .arg("--config")
        .arg(config_path)
        .output()
        .expect("Failed to configure Apollo Router")
}
