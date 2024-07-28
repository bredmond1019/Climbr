use async_graphql::SimpleObject;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

// #[derive(Debug, Serialize, Deserialize, GraphQLObject)]
#[derive(Debug, Serialize, Deserialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct UserDTO {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
