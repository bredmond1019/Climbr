mod config;
pub mod db;
pub mod models;
pub mod schema;

use std::env;
// use std::sync::Arc;

// use actix::Actor;
use actix_cors::Cors;
use actix_web::web;
use actix_web::{middleware::Logger, web::Data, App, HttpServer};

use db::DbPool;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;
use dotenv::dotenv;
use log::info;
use models::user::User;

pub async fn get_user(pool: web::Data<DbPool>) -> Vec<User> {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = pool.get().unwrap();
    let all_users: Result<Vec<User>, diesel::result::Error> =
        web::block(move || User::find_all(&mut conn)).await.unwrap();

    all_users.unwrap()
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect("Error connecting to the database")
}
