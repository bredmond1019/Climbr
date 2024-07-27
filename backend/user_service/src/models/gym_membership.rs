use diesel::{deserialize::Queryable, prelude::Insertable};
use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

use crate::schema::gym_memberships;

#[derive(Queryable, Serialize, Deserialize, GraphQLObject)]
pub struct GymMembership {
    pub id: i32,
    pub user_id: i32,
    pub gym_id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = gym_memberships)]
pub struct NewGymMembership {
    pub user_id: i32,
    pub gym_id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl NewGymMembership {
    pub fn new(user_id: i32, gym_id: i32) -> Self {
        Self {
            user_id: user_id,
            gym_id: gym_id,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}
