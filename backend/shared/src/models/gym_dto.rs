use chrono::{DateTime, Utc};
use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, GraphQLObject)]
#[serde(rename_all = "camelCase")]
pub struct GymDTO {
    id: i32,
    name: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
