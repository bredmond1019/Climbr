pub mod chat_gateway_session;
pub mod chat_service_connection;

use actix::{Addr, Message};
use actix_web::{
    web::{self, ServiceConfig},
    HttpRequest, Responder,
};
use actix_web_actors::ws;
use chat_gateway_session::ChatGatewaySession;
use chat_service_connection::ChatServiceConnection;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("/ws").to(websocket_handler));
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ChatServiceMessage(String);

async fn websocket_handler(
    req: HttpRequest,
    stream: web::Payload,
    chat_connection: web::Data<Addr<ChatServiceConnection>>,
) -> impl Responder {
    let chat_connection_addr = chat_connection.get_ref().clone();

    ws::start(ChatGatewaySession::new(chat_connection_addr), &req, stream)
}
