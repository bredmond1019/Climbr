use std::time::Duration;
use std::{env, thread};

use actix_cors::Cors;
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenv::dotenv;

use auth::authenticator;
use graphql::schema::{create_context, create_schema};
use log::info;
use shared::config;

mod auth;
mod graphql;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "info");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let context = create_context();
    let schema = create_schema(context.clone());

    // Start Apollo Router in a separate thread
    thread::spawn(|| {
        let mut router_process = graphql::start_apollo_router();

        // Monitor the router process
        loop {
            if let Ok(Some(status)) = router_process.try_wait() {
                info!("Apollo Router exited with status: {}", status);
                break;
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    HttpServer::new(move || {
        let auth_middleware = HttpAuthentication::bearer(authenticator);

        App::new()
            // .app_data(Data::new(pool.clone()))
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
