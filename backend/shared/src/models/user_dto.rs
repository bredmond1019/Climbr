use async_graphql::SimpleObject;
// use juniper::GraphQLObject;

use serde::{Deserialize, Serialize};

// #[derive(Debug, Serialize, Deserialize, GraphQLObject)]
#[derive(Debug, Serialize, Deserialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct UserDTO {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
}
