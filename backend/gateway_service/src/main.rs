use std::env;
use std::sync::Arc;

use actix::Actor;
use actix_cors::Cors;
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenv::dotenv;

use auth::auth::authenticator;
use graphql::schema::{create_context, create_schema};
use shared::{config, db};
use websocket::chat_service_connection::{self, ChatServiceConnection};

mod auth;
mod graphql;
mod routes;
mod websocket;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let pool: Pool<ConnectionManager<PgConnection>> = db::init_pool();
    let schema = Arc::new(create_schema());
    let context = create_context(pool.clone());

    let chat_service_addr = "ws://your_chat_service_url".to_string();
    let chat_service_connection = ChatServiceConnection::new(chat_service_addr).start();

    HttpServer::new(move || {
        let auth_middleware = HttpAuthentication::bearer(authenticator);

        App::new()
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(schema.clone()))
            .app_data(Data::new(context.clone()))
            .app_data(Data::new(chat_service_connection.clone()))
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            // .service(routes::login::login)
            // .wrap(auth_middleware)
            .configure(graphql::init)
            .configure(websocket::init)
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
