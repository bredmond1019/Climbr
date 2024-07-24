use chrono::{DateTime, Utc};
use diesel::deserialize::Queryable;
use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, GraphQLObject)]
#[serde(rename_all = "camelCase")]
pub struct AvailabilityDTO {
    pub id: i32,
    pub user_id: i32,
    pub gym_id: i32,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
