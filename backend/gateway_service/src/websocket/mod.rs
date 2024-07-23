pub mod chat_service;

use actix_web::web::ServiceConfig;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(chat_service::websocket_handler);
}
