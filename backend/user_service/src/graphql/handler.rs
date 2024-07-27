use std::sync::Arc;

use actix_web::{web, HttpResponse};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig, GraphiQLSource};
use async_graphql_actix_web::GraphQLRequest;
// use juniper::http::playground::playground_source;
// use juniper::http::GraphQLRequest;

use super::schema::create_context;
use crate::db::DbPool;
use crate::graphql::schema::Schema;

pub async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

pub async fn graphiql() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/graphql").finish())
}

pub async fn graphql_handler(
    pool: web::Data<DbPool>,
    // schema: web::Data<Arc<Schema>>,
    schema: web::Data<Schema<Query, Mutation, EmptySubscription>>,
    // req: HttpRequest,
    data: web::Json<GraphQLRequest>,
) -> HttpResponse {
    schema.execute(req.into_inner()).await.into()
}
