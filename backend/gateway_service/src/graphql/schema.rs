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
        user_service_url: ServiceUrl::User,
        schedule_service_url: ServiceUrl::Schedule,
        client: reqwest::Client::new(),
    }
}

#[derive(Clone, Debug)]
pub enum ServiceUrl {
    User,
    Schedule,
}

impl ServiceUrl {
    pub fn url(&self) -> &str {
        match self {
            ServiceUrl::User => "http://localhost:3001",
            ServiceUrl::Schedule => "http://localhost:3002",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Context {
    pub pool: db::DbPool,
    pub client: reqwest::Client,
    pub user_service_url: ServiceUrl,
    pub schedule_service_url: ServiceUrl,
}

impl juniper::Context for Context {}

impl Context {
    pub fn get_user_service_url(&self) -> &str {
        self.user_service_url.url()
    }

    pub fn get_schedule_service_url(&self) -> &str {
        self.schedule_service_url.url()
    }
}
