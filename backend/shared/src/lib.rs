mod config;
pub mod db;
pub mod models;
pub mod schema;

use std::env;
// use std::sync::Arc;

// use actix::Actor;
use actix_cors::Cors;
use actix_web::{middleware::Logger, web, web::Data, App, HttpServer};

use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "info");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let pool: Pool<ConnectionManager<PgConnection>> = db::init_pool();
    // let schema = Arc::new(create_schema());

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .wrap(Logger::default())
            .wrap(Cors::permissive())
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
