use crate::db::DbPool;
use crate::models::messages::{ClientMessage, MessageTarget, NewMessage};
use crate::schema::messages::dsl::*;
use actix::prelude::*;
use actix_web::web;
use actix_web_actors::ws;

use diesel::prelude::*;
use uuid::Uuid;

pub struct WsChatSession {
    pub id: Uuid,
    pub db_pool: web::Data<DbPool>,
}

impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("WebSocket connection established: {:?}", self.id);
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        println!("WebSocket connection closed: {:?}", self.id);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(client_message_json)) => {
                println!("Received message: {}", client_message_json);

                let client_message: ClientMessage =
                    serde_json::from_str(&client_message_json).unwrap();
                let ClientMessage {
                    sender_id: s_id,
                    receiver_id: r_id,
                    content: message,
                } = client_message;

                println!("Received message from {}: {}", s_id, message);

                let new_message =
                    NewMessage::new(message.to_string(), s_id, MessageTarget::User(r_id));

                let mut conn = self.db_pool.get().expect("Failed to get DB connection");
                diesel::insert_into(messages)
                    .values(&new_message)
                    .execute(&mut conn)
                    .expect("Failed to insert message");

                ctx.text(client_message_json);
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Pong(_)) => (),
            Ok(ws::Message::Close(reason)) => ctx.close(reason),
            _ => (),
        }
    }
}
