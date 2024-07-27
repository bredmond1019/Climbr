use diesel::{deserialize::Queryable, prelude::Insertable};
use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

use crate::schema::gyms;

#[derive(Queryable, Serialize, Deserialize, GraphQLObject)]
pub struct Gym {
    pub id: i32,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = gyms)]
pub struct NewGym {
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl NewGym {
    pub fn new(name: String) -> Self {
        Self {
            name: name,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}
