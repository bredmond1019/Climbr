use juniper::{EmptySubscription, RootNode};

use super::mutation::Mutation;
use crate::{auth::UserResponse, db, graphql::query::Query};

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}

pub fn create_context(pool: db::DbPool) -> Context {
    Context { pool, user: None }
}

#[derive(Clone, Debug)]
pub struct Context {
    pub pool: db::DbPool,
    pub user: Option<UserResponse>,
}

impl juniper::Context for Context {}
