use std::env;
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use auth::authenticator;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenv::dotenv;
use graphql::schema::{create_context, create_schema};

use shared::{config, db};
use tokio::sync::Mutex;
mod auth;
mod graphql;
mod models;
mod routes;
mod schema;
mod seed;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let pool: Pool<ConnectionManager<PgConnection>> = db::init_pool();
    let schema = Arc::new(create_schema());
    let context = Arc::new(Mutex::new(create_context(pool.clone())));

    HttpServer::new(move || {
        let auth_middleware = HttpAuthentication::bearer(authenticator);

        seed::seed_dev_data(&mut pool.get().expect("Error connecting to database"));

        App::new()
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(schema.clone()))
            .app_data(Data::new(context.clone()))
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            // .wrap(auth_middleware)
            .configure(graphql::init)
    })
    .bind("0.0.0.0:3001")?
    .run()
    .await
}
