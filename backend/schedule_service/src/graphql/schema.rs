use async_graphql::{EmptySubscription, Schema};
use shared::db;

use super::{mutation::Mutation, query::Query};

pub fn create_schema(ctx: AppContext) -> Schema<Query, Mutation, EmptySubscription> {
    Schema::build(Query, Mutation, EmptySubscription)
        .data(ctx.clone())
        .finish()
}

pub fn create_context(pool: db::DbPool) -> AppContext {
    AppContext { pool }
}

#[derive(Clone, Debug)]
pub struct AppContext {
    pub pool: db::DbPool,
}
