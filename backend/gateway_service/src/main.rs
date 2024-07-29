use std::env;
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenv::dotenv;

use auth::authenticator;
use graphql::schema::{create_context, create_schema};
use shared::{config, db};

mod auth;
mod graphql;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let pool: Pool<ConnectionManager<PgConnection>> = db::init_pool();
    let context = create_context(pool.clone());
    let schema = create_schema(context.clone());

    HttpServer::new(move || {
        let auth_middleware = HttpAuthentication::bearer(authenticator);

        App::new()
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(schema.clone()))
            .app_data(Data::new(context.clone()))
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .configure(routes::init_routes)
            // .wrap(auth_middleware)
            .configure(graphql::init)
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
