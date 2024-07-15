use actix::Message;
use chrono::NaiveDateTime;
use diesel::{deserialize::Queryable, prelude::*, sql_types::Uuid, PgConnection, RunQueryDsl};

use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

use crate::{
    chat::chat_server::{ReceiverId, SessionId},
    schema::messages,
};

#[derive(Debug, Serialize, Deserialize)]
pub enum MessageTarget {
    User(i32),
    Channel(i32),
}

#[derive(Debug, Serialize, Deserialize, Queryable, Clone, GraphQLObject)]
#[diesel(table_name = crate::schema::messages)]
pub struct ChatMessage {
    pub id: i32,
    pub conversation_id: i32,
    pub sender_id: i32,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = messages)]
pub struct NewMessage {
    pub conversation_id: i32,
    pub sender_id: i32,
    pub content: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Deserialize, Serialize, Message, Clone, Debug)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub sender_id: i32,
    pub receiver_id: ReceiverId,
    pub content: String,
    pub session_id: SessionId,
}

impl NewMessage {
    pub fn new(content: String, sender_id: i32, conversation_id: i32) -> Self {
        NewMessage {
            conversation_id,
            sender_id,
            content,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

impl ChatMessage {
    pub fn create(
        new_message: NewMessage,
        conn: &mut PgConnection,
    ) -> Result<ChatMessage, diesel::result::Error> {
        let message = diesel::insert_into(messages::table)
            .values(&new_message)
            .get_result(conn);
        message
    }
}
