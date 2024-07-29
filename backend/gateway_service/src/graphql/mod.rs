use std::{
    env,
    process::{Child, Command, Output},
};

use actix_web::web::{self, ServiceConfig};
use config::{config_apollo_router, publish_schedule_service_schema, publish_user_service_schema};
use handler::graphql_handler;

mod config;
mod handler;
mod mutation;
mod query;
mod utils;

pub mod schema;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(
        web::resource("/graphql")
            .route(web::get().to(graphql_handler))
            .route(web::post().to(graphql_handler)),
    )
    .service(web::resource("/playground").route(web::get().to(handler::graphql_playground)))
    .service(web::resource("/graphiql").route(web::get().to(handler::graphiql)));
}

pub fn setup_apollo_router() -> Output {
    publish_schedule_service_schema();
    publish_user_service_schema();
    config_apollo_router()
}

pub fn start_apollo_router() -> Child {
    let router_path = env::var("APOLLO_ROUTER_PATH").expect("APOLLO_ROUTER_PATH must be set");
    let supergraph_path = env::var("SUPERGRAPH_PATH").expect("SUPERGRAPH_PATH must be set");

    Command::new(router_path)
        .arg("--dev")
        .arg("--supergraph")
        .arg(supergraph_path)
        .spawn()
        .expect("Failed to start Apollo Router")
}
