use actix::Message;
use chrono::NaiveDateTime;
use diesel::{deserialize::Queryable, prelude::*, PgConnection, RunQueryDsl};

use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

use crate::{chat::chat_server::ConversationId, schema::chat_messages};

use super::conversation::Conversation;
use super::user::User;

#[derive(Debug, Serialize, Deserialize, Queryable, Clone, GraphQLObject, Associations)]
#[diesel(table_name = crate::schema::chat_messages)]
#[diesel(belongs_to(Conversation))]
#[diesel(belongs_to(User))]
pub struct ChatMessage {
    pub id: i32,
    pub conversation_id: i32,
    pub user_id: i32,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = chat_messages)]
pub struct NewChatMessage {
    pub conversation_id: i32,
    pub user_id: i32,
    pub content: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl NewChatMessage {
    pub fn new(content: String, user_id: i32, conversation_id: i32) -> Self {
        NewChatMessage {
            conversation_id,
            user_id,
            content,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

impl ChatMessage {
    pub fn create(
        new_message: NewChatMessage,
        conn: &mut PgConnection,
    ) -> Result<ChatMessage, diesel::result::Error> {
        let message = diesel::insert_into(chat_messages::table)
            .values(&new_message)
            .get_result(conn);
        message
    }
}

// TODO: This probably is Redundant and should be removed
#[derive(Deserialize, Serialize, Message, Clone, Debug)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub sender_id: i32,
    pub receiver_id: i32,
    pub content: String,
    pub conversation_id: ConversationId,
}
