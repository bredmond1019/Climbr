use std::env;
use std::sync::Arc;

use actix::Actor;
use actix_cors::Cors;
use actix_web::{middleware::Logger, web, web::Data, App, HttpServer};
use chat::chat_server::ChatServer;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenv::dotenv;
use graphql::handlers::{graphiql, graphql_handler, graphql_playground};
use graphql::schema::create_schema;

mod auth;
mod chat;
mod config;
mod db;
mod graphql;
mod models;
mod routes;
mod schema;
mod websocket;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "info");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let pool: Pool<ConnectionManager<PgConnection>> = db::init_pool();
    let schema = Arc::new(create_schema());
    let chat_server = ChatServer::new(pool.clone()).start();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(schema.clone()))
            .app_data(Data::new(chat_server.clone()))
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .service(
                web::resource("/graphql")
                    .route(web::get().to(graphql_handler))
                    .route(web::post().to(graphql_handler)),
            )
            .service(web::resource("/playground").route(web::get().to(graphql_playground)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
            .configure(routes::init_routes)
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
