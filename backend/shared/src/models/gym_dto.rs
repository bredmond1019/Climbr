use async_graphql::SimpleObject;
// use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct GymDTO {
    id: i32,
    name: String,
    created_at: String,
    updated_at: String,
}
