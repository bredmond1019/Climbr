use std::{
    env,
    process::{Child, Command, Output},
};

use actix_web::{
    error::ErrorInternalServerError,
    web::{self, ServiceConfig},
    HttpMessage,
};

use config::proxy_to_apollo_router;

// use handler::graphql_handler;

mod config;
mod handler;
mod mutation;
mod query;
mod utils;

pub mod schema;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(
        web::resource("/graphql").route(web::to(proxy_to_apollo_router)), // Proxy requests to Apollo Router
                                                                          // .route(web::get().to(graphql_handler))
                                                                          // .route(web::post().to(graphql_handler)),
    )
    .service(web::resource("/playground").route(web::get().to(handler::graphql_playground)))
    .service(web::resource("/graphiql").route(web::get().to(handler::graphiql)));
}

pub fn start_apollo_router() -> Child {
    let router_path = env::var("APOLLO_ROUTER_PATH").expect("APOLLO_ROUTER_PATH must be set");
    let router_config_path =
        env::var("APOLLO_ROUTER_CONFIG_PATH").expect("APOLLO_ROUTER_CONFIG_PATH must be set");
    let supergraph_path =
        env::var("SUPERGRAPH_SCHEMA_PATH").expect("SUPERGRAPH_SCHEMA_PATH must be set");

    Command::new(router_path)
        .arg("--config")
        .arg(router_config_path)
        .arg("--dev")
        .arg("--supergraph")
        .arg(supergraph_path)
        .spawn()
        .expect("Failed to start Apollo Router")
}
