use async_graphql::SimpleObject;
// use juniper::GraphQLObject;

use serde::{Deserialize, Serialize};

use super::datetime::DateTimeUTC;

// #[derive(Debug, Serialize, Deserialize, GraphQLObject)]
#[derive(Debug, Serialize, Deserialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct UserDTO {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: DateTimeUTC,
    pub updated_at: DateTimeUTC,
}
