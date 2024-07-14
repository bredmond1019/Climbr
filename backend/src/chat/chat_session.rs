use crate::{
    chat::chat_server::ChatServer,
    models::{message::ClientMessage, user},
};
use actix::prelude::*;
use actix_web_actors::ws;
use serde::Deserialize;
use uuid::Uuid;

use super::chat_server::SessionId;

pub struct ChatSession {
    pub id: SessionId,
    pub addr: Addr<ChatServer>,
    pub members: Option<[ChatSessionMember; 2]>,
    pub conversation_id: Option<i32>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ChatServerConnect {
    pub chat_session_id: SessionId,
    pub addr: Addr<ChatSession>,
    pub members: [ChatSessionMember; 2], // Array of two members
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ChatServerDisconnect {
    pub chat_session_id: SessionId,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InitialMessage {
    pub user_id1: i32,
    pub user_id2: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChatSessionMember {
    pub user_id: i32,
    pub user_addr: Addr<ChatSession>,
}

impl ChatSession {
    pub fn new(addr: Addr<ChatServer>) -> Self {
        ChatSession {
            id: SessionId(Uuid::new_v4()),
            addr,
            members: None,
            conversation_id: None,
        }
    }
}

impl Actor for ChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // TODO: Display what the Message Data should look like
        ctx.text("Initiated Chat Session");
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.addr.do_send(ChatServerDisconnect {
            chat_session_id: self.id,
        });
        Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                if self.members.is_none() {
                    match serde_json::from_str::<InitialMessage>(&text) {
                        Ok(initial_msg) => {
                            let user1_id = initial_msg.user_id1;
                            let user2_id = initial_msg.user_id2;
                            let session_address = ctx.address();

                            let member1 = ChatSessionMember {
                                user_id: user1_id,
                                user_addr: session_address.clone(),
                            };
                            let member2 = ChatSessionMember {
                                user_id: user2_id,
                                user_addr: session_address.clone(),
                            };

                            self.members = Some([member1, member2]);

                            let name =
                                format!("Conversation between {:?} and {:?}", user1_id, user2_id);

                            self.addr.do_send(ChatServerConnect {
                                chat_session_id: self.id,
                                addr: session_address.clone(),
                                members: self.members.clone().expect("Members not set"),
                            });
                        }
                        Err(err) => {
                            // Error if InitialMessage doesn't include user_id1 and user_id2
                            eprintln!("Error deserializing InitialMessage: {:?}", err);
                        }
                    }
                } else {
                    let mut message: ClientMessage = serde_json::from_str(&text).unwrap();
                    message.conversation_id = self.conversation_id.unwrap();
                    self.addr.do_send(message);
                }
            }
            _ => (),
        }
    }
}

impl Handler<ClientMessage> for ChatSession {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, ctx: &mut Self::Context) {
        // Serialize the message to JSON
        let response = serde_json::to_string(&msg).unwrap();
        // Send the JSON message to the WebSocket client
        ctx.text(response);
    }
}
