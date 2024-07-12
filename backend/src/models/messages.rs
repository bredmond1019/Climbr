use chrono::NaiveDateTime;
use diesel::{deserialize::Queryable, prelude::*, PgConnection, RunQueryDsl};

use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

use crate::schema::messages;

#[derive(Debug, Serialize, Deserialize)]
pub enum MessageTarget {
    User(i32),
    Channel(i32),
}

#[derive(Debug, Serialize, Deserialize, Queryable, Clone, GraphQLObject)]
#[diesel(table_name = crate::schema::messages)]
pub struct Message {
    pub id: i32,
    pub sender_id: i32,
    pub receiver_id: Option<i32>,
    pub channel_id: Option<i32>,
    pub content: String,
    pub timestamp: NaiveDateTime,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = messages)]
pub struct NewMessage {
    content: String,
    sender_id: i32,
    receiver_id: Option<i32>,
    channel_id: Option<i32>,
    timestamp: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct ClientMessage {
    pub sender_id: i32,
    pub receiver_id: i32,
    pub content: String,
}

impl NewMessage {
    pub fn new(content: String, sender_id: i32, target: MessageTarget) -> Self {
        let (receiver_id, channel_id) = match target {
            MessageTarget::User(id) => (Some(id), None),
            MessageTarget::Channel(id) => (None, Some(id)),
        };

        NewMessage {
            content,
            sender_id,
            receiver_id,
            channel_id,
            timestamp: chrono::Local::now().naive_local(),
        }
    }
}

impl Message {
    pub fn create(
        new_message: NewMessage,
        conn: &mut PgConnection,
    ) -> Result<Message, diesel::result::Error> {
        let message = diesel::insert_into(messages::table)
            .values(&new_message)
            .get_result(conn);
        message
    }
}
