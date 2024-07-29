use async_graphql::SimpleObject;
use chrono::{NaiveDateTime, Utc};
use diesel::{deserialize::Queryable, prelude::Insertable};

use serde::{Deserialize, Serialize};
use shared::models::datetime::DateTimeUTC;

use crate::schema::gyms;

#[derive(Queryable, Serialize, Deserialize, SimpleObject)]
pub struct Gym {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = gyms)]
pub struct NewGym {
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl NewGym {
    pub fn new(name: String) -> Self {
        let now = Utc::now().naive_utc();
        Self {
            name: name,
            created_at: now,
            updated_at: now,
        }
    }
}
