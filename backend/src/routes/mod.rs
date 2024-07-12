mod home;
mod login;
mod user;
mod websocket;

use actix_web::web;
use login as Log;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(home::index);
    cfg.service(user::create_user);
    cfg.service(user::get_user);
    cfg.service(Log::login);
    cfg.route("/ws/", web::get().to(websocket::ws_index));
}
