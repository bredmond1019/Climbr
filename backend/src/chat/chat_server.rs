use std::collections::HashMap;

use crate::db::DbPool;
use crate::models::message::ClientMessage;
use actix::{Actor, Addr, Context, Handler};
use log::info;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::chat_session::{ChatServerConnect, ChatServerDisconnect, ChatSession};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct ReceiverId(i32);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SessionId(pub Uuid);

pub struct ChatServer {
    sessions: HashMap<SessionId, Addr<ChatSession>>,
    user_sessions: HashMap<ReceiverId, Vec<SessionId>>,
    db_pool: DbPool,
}

impl ChatServer {
    pub fn new(db_pool: DbPool) -> Self {
        ChatServer {
            sessions: HashMap::new(),
            user_sessions: HashMap::new(),
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
        info!("Members: {:?}", msg.members);
        self.sessions.insert(msg.chat_session_id, msg.addr);
        for member in msg.members.iter() {
            let member_id = ReceiverId(member.conversation_member.user_id);
            self.user_sessions
                .entry(member_id)
                .or_default()
                .push(msg.chat_session_id);
        }
    }
}

impl Handler<ClientMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Self::Context) {
        // let conn = self.db_pool.get().expect("Failed to get DB connection");

        // let new_message = NewMessage::new(msg.content.clone(), msg.sender_id, msg.conversation_id);

        if let Some(receivers) = self.user_sessions.get(&msg.receiver_id) {
            for &session_id in receivers {
                if let Some(addr) = self.sessions.get(&session_id) {
                    info!("Sending message to session: {:?}", msg);
                    addr.do_send(msg.clone());
                }
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

// {
// 	"user_id1": 1,
// 	"user_id2": 2
// }
