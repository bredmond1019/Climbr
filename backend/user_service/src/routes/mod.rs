use actix_web::web;

mod user;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(user::create_user);
    cfg.service(user::get_user);
}
