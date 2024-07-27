use actix_web::web;
use async_graphql::{EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use super::{mutation::Mutation, query::Query};

pub async fn graphql_handler(
    schema: web::Data<Schema<Query, Mutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

// async fn playground_handler() -> actix_web::Result<actix_files::NamedFile> {
//     Ok(actix_files::NamedFile::open("./playground.html")?)
// }
