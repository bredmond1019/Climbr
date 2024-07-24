use std::sync::Arc;

use actix_web::{web, HttpResponse};
use juniper::http::graphiql::graphiql_source;
use juniper::http::playground::playground_source;
use juniper::http::GraphQLRequest;
use log::info;

use super::schema::create_context;
use crate::db::DbPool;
use crate::graphql::schema::Schema;

pub async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source("/graphql", None))
}

pub async fn graphiql() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(graphiql_source("/graphql", None))
}

pub async fn graphql_handler(
    pool: web::Data<DbPool>,
    schema: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> HttpResponse {
    let ctx = create_context(pool.get_ref().clone());

    info!("Executing query: {:?}", data);

    let res = data.execute(&schema, &ctx).await;
    info!("Response: {:?}", res);
    HttpResponse::Ok().json(res)
}
