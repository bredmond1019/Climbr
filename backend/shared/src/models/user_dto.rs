use chrono::{DateTime, Utc};
// use juniper::GraphQLObject;

use serde::{Deserialize, Serialize};

// #[derive(Debug, Serialize, Deserialize, GraphQLObject)]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDTO {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
