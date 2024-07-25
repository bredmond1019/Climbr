use std::env;

use diesel::{Connection, PgConnection};
use dotenv::dotenv;

pub mod config;
pub mod db;
pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect("Error connecting to the database")
}
