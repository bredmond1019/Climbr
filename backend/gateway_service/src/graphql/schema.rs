use juniper::{EmptySubscription, RootNode};

use super::{mutation::Mutation, query::Query};
use crate::db;

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}

pub fn create_context(pool: db::DbPool) -> Context {
    Context {
        pool: pool,
        user_service_url: "http://localhost:3001".to_string(),
        schedule_service_url: "http://localhost:3002".to_string(),
        client: reqwest::Client::new(),
    }
}

#[derive(Clone, Debug)]
pub struct Context {
    pub pool: db::DbPool,
    pub user_service_url: String,
    pub schedule_service_url: String,
    pub client: reqwest::Client,
}

impl juniper::Context for Context {}
