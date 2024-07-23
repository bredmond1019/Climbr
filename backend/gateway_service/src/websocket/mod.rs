pub mod chat_service;

use actix_web::web::{self, ServiceConfig};
use chat_service::websocket_handler;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("/ws").to(websocket_handler));
}
