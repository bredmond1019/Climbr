use async_graphql::SimpleObject;
use chrono::{NaiveDateTime, Utc};
use diesel::{deserialize::Queryable, prelude::Insertable};

use serde::{Deserialize, Serialize};
use shared::models::datetime::DateTimeUTC;

use crate::schema::gym_memberships;

#[derive(Queryable, Serialize, Deserialize, SimpleObject)]
pub struct GymMembership {
    pub id: i32,
    pub user_id: i32,
    pub gym_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = gym_memberships)]
pub struct NewGymMembership {
    pub user_id: i32,
    pub gym_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl NewGymMembership {
    pub fn new(user_id: i32, gym_id: i32) -> Self {
        let now = Utc::now().naive_utc();
        Self {
            user_id: user_id,
            gym_id: gym_id,
            created_at: now,
            updated_at: now,
        }
    }
}
