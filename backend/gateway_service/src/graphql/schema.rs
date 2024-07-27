use async_graphql::{EmptySubscription, Schema};
use shared::db;

use super::{mutation::Mutation, query::Query};

pub fn create_schema(ctx: AppContext) -> Schema<Query, Mutation, EmptySubscription> {
    Schema::build(Query, Mutation, EmptySubscription)
        .data(ctx.clone())
        .finish()
}

pub fn create_context(pool: db::DbPool) -> AppContext {
    AppContext {
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
pub struct AppContext {
    pub pool: db::DbPool,
    pub client: reqwest::Client,
    pub user_service_url: ServiceUrl,
    pub schedule_service_url: ServiceUrl,
}

// impl async_graphql::Context for AppContext {}

impl AppContext {
    pub fn get_user_service_url(&self) -> &str {
        self.user_service_url.url()
    }

    pub fn get_schedule_service_url(&self) -> &str {
        self.schedule_service_url.url()
    }
}
