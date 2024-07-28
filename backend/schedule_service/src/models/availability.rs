use async_graphql::SimpleObject;
use chrono::{NaiveDateTime, Utc};
use diesel::RunQueryDsl;
use diesel::{deserialize::Queryable, prelude::Insertable, PgConnection, Selectable};

use serde::{Deserialize, Serialize};

use crate::schema::availabilities;

#[derive(Queryable, Serialize, Deserialize, SimpleObject, Selectable)]
#[diesel(table_name = crate::schema::availabilities)]
pub struct Availability {
    pub id: i32,
    pub user_id: i32,
    pub gym_id: i32,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = availabilities)]
pub struct NewAvailability {
    pub user_id: i32,
    pub gym_id: i32,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl NewAvailability {
    pub fn new(
        user_id: i32,
        gym_id: i32,
        start_time: NaiveDateTime,
        end_time: NaiveDateTime,
    ) -> Self {
        let now = Utc::now().naive_utc();
        NewAvailability {
            user_id,
            gym_id,
            start_time,
            end_time,
            created_at: now,
            updated_at: now,
        }
    }
}

impl Availability {
    pub fn create(new_availability: NewAvailability, conn: &mut PgConnection) -> Self {
        diesel::insert_into(availabilities::table)
            .values(&new_availability)
            .get_result(conn)
            .expect("Error creating availability")
    }
}
