use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse};
use juniper::http::graphiql::graphiql_source;
use juniper::http::playground::playground_source;
use juniper::http::GraphQLRequest;
use log::info;

use crate::graphql::schema::Schema;

use crate::db::DbPool;

use super::schema::create_context;

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
    req: HttpRequest,
    data: web::Json<GraphQLRequest>,
) -> HttpResponse {
    let mut ctx = create_context(pool.get_ref().clone());

    // if !is_login_mutation {
    //     if let Err(e) = authenticate(&req, &mut ctx) {
    //         return e.into();
    //     }
    // }

    let res = data.execute(&schema, &ctx).await;
    HttpResponse::Ok().json(res)
}
