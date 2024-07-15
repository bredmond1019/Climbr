use crate::models::{
    conversation::Conversation, conversation_member::ConversationMember, message::ClientMessage,
};
use crate::ChatServer;
use actix::prelude::*;
use actix_web_actors::ws;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use serde::Deserialize;
use uuid::Uuid;

use super::chat_server::SessionId;

pub struct ChatSession {
    pub id: SessionId,
    pub addr: Addr<ChatServer>,
    pub members: Vec<ChatSessionMember>,
    pub conversation_id: i32,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ChatServerConnect {
    pub chat_session_id: SessionId,
    pub addr: Addr<ChatSession>,
    pub members: Vec<ChatSessionMember>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ChatServerDisconnect {
    pub chat_session_id: SessionId,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InitialMessage {
    pub user_ids: Vec<i32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChatSessionMember {
    pub conversation_member: ConversationMember,
    pub user_addr: Option<Addr<ChatSession>>,
}

impl ChatSession {
    pub fn new(
        addr: Addr<ChatServer>,
        conversation: Conversation,
        connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    ) -> Self {
        let members = conversation
            .members(connection)
            .expect("Failed to get conversation members")
            .iter()
            .map(|member| ChatSessionMember {
                conversation_member: member.clone(),
                user_addr: None,
            })
            .collect::<Vec<ChatSessionMember>>();

        ChatSession {
            id: SessionId(Uuid::new_v4()),
            addr,
            members,
            conversation_id: conversation.id,
        }
    }
}

impl Actor for ChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.members.iter_mut().for_each(|member| {
            member.user_addr = Some(ctx.address().clone());
        });

        self.addr.do_send(ChatServerConnect {
            chat_session_id: self.id,
            addr: ctx.address().clone(),
            members: self.members.clone(),
        });

        ctx.text("Initiated Chat Session");
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        self.addr.do_send(ChatServerDisconnect {
            chat_session_id: self.id,
        });
        ctx.text("Chat Session Stopped");
        Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                let message: ClientMessage = match serde_json::from_str(&text) {
                    Ok(msg) => msg,
                    Err(err) => {
                        // Error if failed to deserialize ClientMessage
                        let error_message = format!("Error deserializing ClientMessage: {:?}", err);
                        ctx.text(error_message);
                        return;
                    }
                };

                self.addr.do_send(message);
            }
            _ => (),
        }
    }
}

impl Handler<ClientMessage> for ChatSession {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, ctx: &mut Self::Context) {
        // Serialize the message to JSON
        let response = serde_json::to_string(&msg).expect("Failed to serialize ClientMessage");
        // Send the JSON message to the WebSocket client
        ctx.text(response);
    }
}
