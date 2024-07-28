use async_graphql::SimpleObject;
use diesel::{deserialize::Queryable, prelude::Insertable};

use serde::{Deserialize, Serialize};
use shared::models::datetime::DateTimeUTC;

use crate::schema::gyms;

#[derive(Queryable, Serialize, Deserialize, SimpleObject)]
pub struct Gym {
    pub id: i32,
    pub name: String,
    pub created_at: DateTimeUTC,
    pub updated_at: DateTimeUTC,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = gyms)]
pub struct NewGym {
    pub name: String,
    pub created_at: DateTimeUTC,
    pub updated_at: DateTimeUTC,
}

impl NewGym {
    pub fn new(name: String) -> Self {
        Self {
            name: name,
            created_at: DateTimeUTC(chrono::Utc::now()),
            updated_at: DateTimeUTC(chrono::Utc::now()),
        }
    }
}
