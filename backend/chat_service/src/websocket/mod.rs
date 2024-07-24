use actix::Addr;
use actix_web::{
    web::{self, ServiceConfig},
    HttpRequest, Responder,
};
use actix_web_actors::ws;
use chat_server::ChatServer;
use chat_session::ChatSession;

use crate::db::DbPool;
use crate::models::conversation::Conversation;

pub mod chat_server;
pub mod chat_session;
mod utils;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("/ws").to(websocket_handler));
}

pub async fn websocket_handler(
    req: HttpRequest,
    stream: web::Payload,
    server: web::Data<Addr<ChatServer>>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection");
    let chat_server_address = server.get_ref().clone();
    let (sender_id, conversation_member_ids, conversation_id) =
        utils::get_session_data(req.query_string());

    let conversation =
        Conversation::find_or_create(&mut conn, conversation_id, conversation_member_ids);
    let session_member = conversation.find_membership_by_user_id(sender_id, &mut conn);

    let session = ChatSession::new(chat_server_address, conversation, session_member);
    ws::start(session, &req, stream)
}
