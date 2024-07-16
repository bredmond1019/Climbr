use actix::Addr;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use log::info;

use crate::chat::chat_session::{ChatSession, InitiateChatMessage};
use crate::db::DbPool;
use crate::models::conversation::Conversation;

use crate::ChatServer;

pub async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    server: web::Data<Addr<ChatServer>>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    let query_string = req.query_string();
    info!("Query: {}", query_string);

    let initial_message: Result<InitiateChatMessage, _> = serde_qs::from_str(query_string);
    match initial_message {
        Ok(initial_msg) => {
            let mut conn = pool.get().expect("Failed to get DB connection");
            let chat_server_address = server.get_ref().clone();

            let user_id = initial_msg.user_id;
            let receiver_id = initial_msg.receiver_id;
            let user_ids = vec![user_id, receiver_id];

            let conversation_id = initial_msg.conversation_id;
            let conversation = Conversation::find_or_create(&mut conn, conversation_id, user_ids);

            let conversation_members = conversation.members(&mut conn);
            let session_member = conversation_members
                .iter()
                .find(|member| member.user_id == user_id)
                .expect("Failed to find conversation member");

            let session =
                ChatSession::new(chat_server_address, conversation, session_member.clone());
            ws::start(session, &req, stream)
        }

        Err(err) => {
            info!("Error parsing query string: {:?}", err);
            Ok(HttpResponse::BadRequest().body("Invalid Initial Query String"))
        }
    }
}
