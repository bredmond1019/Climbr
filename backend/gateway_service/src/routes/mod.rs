use actix_web::web;

mod home;
pub mod login;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(home::index);
    cfg.service(login::user_login);
}
