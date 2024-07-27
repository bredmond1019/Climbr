use async_graphql::SimpleObject;
use diesel::deserialize::Queryable;
// use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct AvailabilityDTO {
    pub id: i32,
    pub user_id: i32,
    pub gym_id: i32,
    pub start_time: String,
    pub end_time: String,
    pub created_at: String,
    pub updated_at: String,
}
