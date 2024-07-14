use actix::Addr;
use actix_web::{web, HttpRequest, Responder};
use actix_web_actors::ws;

use crate::chat::{chat_server::ChatServer, chat_session::ChatSession};

pub async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    server: web::Data<Addr<ChatServer>>,
) -> impl Responder {
    let chat_server_address = server.get_ref().clone();
    let session = ChatSession::new(chat_server_address);
    ws::start(session, &req, stream)
}
