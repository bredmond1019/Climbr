mod home;
pub mod login;

use actix_web::web;
// use login as Log;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(home::index);
    // cfg.service(Log::login);
}
