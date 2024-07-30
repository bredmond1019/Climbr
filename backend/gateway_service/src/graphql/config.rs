use actix_web::error::ErrorInternalServerError;
use actix_web::{web, Error, HttpMessage, HttpRequest, HttpResponse};
use log::{error, info};
use std::env;
use std::io::Write;
use std::process::{Command, Output};

pub async fn proxy_to_apollo_router(
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse, Error> {
    let client = awc::Client::default();

    let mut forward_req = client
        .request_from("http://localhost:4000/graphql", req.head())
        .no_decompress();

    for (key, value) in req.headers().iter() {
        forward_req.headers_mut().insert(key.clone(), value.clone());
    }

    let mut res = forward_req
        .send_stream(payload)
        .await
        .map_err(|e| ErrorInternalServerError(e))?;

    let mut client_resp = HttpResponse::build(res.status());

    for (key, value) in res.headers().iter() {
        client_resp.insert_header((key.clone(), value.clone()));
    }

    Ok(client_resp.streaming(res.take_payload()))
}

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

    info!("Published schema for subgraph: {}", subgraph_name);
    info!("Schema path: {}", schema_path);
    info!("Routing URL: {}", routing_url);
    info!("Graph ID: {}", graph_id);

    let rover_path = env::var("ROVER_PATH").expect("ROVER_PATH must be set");

    let mut child = Command::new(rover_path)
        .arg("subgraph")
        .arg("publish")
        .arg(graph_id)
        .arg("--name")
        .arg(subgraph_name)
        .arg("--schema")
        .arg(schema_path)
        .arg("--routing-url")
        .arg(routing_url)
        .spawn()
        .expect("Failed to start command");

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(b"y\n").expect("Failed to write to stdin");
    }

    let output = child.wait_with_output().expect("Failed to read stdout");

    info!("Output: {:?}", output);
    output
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
