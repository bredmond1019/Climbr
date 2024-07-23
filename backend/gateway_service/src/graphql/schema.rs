use juniper::{EmptySubscription, RootNode};

use super::{mutation::Mutation, query::Query};
use crate::db;

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}

pub fn create_context(pool: db::DbPool) -> Context {
    Context { pool }
}

#[derive(Clone, Debug)]
pub struct Context {
    pub pool: db::DbPool,
}

impl juniper::Context for Context {}
