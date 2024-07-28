use actix_web::web::{self, ServiceConfig};
use handler::graphql_handler;

pub mod handler;
pub mod mutation;
pub mod query;
pub mod schema;
pub mod utils;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(
        web::resource("/graphql")
            .route(web::get().to(graphql_handler))
            .route(web::post().to(graphql_handler)),
    )
    .service(web::resource("/playground").route(web::get().to(handler::graphql_playground)))
    .service(web::resource("/graphiql").route(web::get().to(handler::graphiql)));
}
