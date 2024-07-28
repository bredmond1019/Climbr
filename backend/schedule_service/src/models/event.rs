use async_graphql::SimpleObject;
use chrono::NaiveDateTime;
use diesel::RunQueryDsl;
use diesel::{deserialize::Queryable, prelude::Insertable, PgConnection, Selectable};
use serde::{Deserialize, Serialize};

use crate::schema::events;

#[derive(Queryable, Serialize, Deserialize, SimpleObject, Selectable)]
#[diesel(table_name = crate::schema::events)]
pub struct Event {
    pub id: i32,
    pub gym_id: i32,
    pub requester_id: i32,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = events)]
pub struct NewEvent {
    pub gym_id: i32,
    pub requester_id: i32,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl NewEvent {
    pub fn new(
        gym_id: i32,
        requester_id: i32,
        start_time: NaiveDateTime,
        end_time: NaiveDateTime,
    ) -> Self {
        NewEvent {
            gym_id,
            requester_id,
            start_time,
            end_time,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}

impl Event {
    pub fn create(new_event: NewEvent, conn: &mut PgConnection) -> Self {
        diesel::insert_into(events::table)
            .values(&new_event)
            .get_result(conn)
            .expect("Error creating event")
    }
}
