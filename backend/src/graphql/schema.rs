use crate::{db, graphql::query::Query};
use juniper::{EmptyMutation, EmptySubscription, RootNode};

pub type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, EmptyMutation::new(), EmptySubscription::new())
}

pub fn create_context(pool: db::DbPool) -> Context {
    Context { pool }
}

#[derive(Clone)]
pub struct Context {
    pub pool: db::DbPool,
}

impl juniper::Context for Context {}
