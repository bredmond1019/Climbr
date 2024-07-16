use std::collections::HashMap;

use crate::db::DbPool;
use crate::models::message::ClientMessage;
use actix::{Actor, Addr, Context, Handler};
use log::info;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::chat_session::{ChatServerConnect, ChatServerDisconnect, ChatSession};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct ConversationId(pub i32);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SessionId(pub Uuid);

pub struct ChatServer {
    sessions: HashMap<SessionId, Addr<ChatSession>>,
    conversation_sessions: HashMap<ConversationId, Vec<Addr<ChatSession>>>,
    db_pool: DbPool,
}

impl ChatServer {
    pub fn new(db_pool: DbPool) -> Self {
        ChatServer {
            sessions: HashMap::new(),
            conversation_sessions: HashMap::new(),
            db_pool,
        }
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}

impl Handler<ChatServerConnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ChatServerConnect, _: &mut Self::Context) {
        info!("Adding chat session: {:?}", msg.chat_session_id);
        // Insert the chat session into the sessions HashMap
        self.sessions.insert(msg.chat_session_id, msg.addr.clone());
        self.conversation_sessions
            .entry(msg.conversation_id)
            .or_insert_with(Vec::new)
            .push(msg.addr);
    }
}

impl Handler<ClientMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Self::Context) {
        if let Some(sessions) = self.conversation_sessions.get(&msg.conversation_id) {
            for session in sessions {
                session.do_send(msg.clone());
            }
        }
    }
}

impl Handler<ChatServerDisconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ChatServerDisconnect, _: &mut Self::Context) {
        self.sessions.remove(&msg.chat_session_id);
    }
}

// {
//     "sender_id": 1,
//     "receiver_id": 2,
//     "content": "Hello, world!",
//     "conversation_id": 1
// }
