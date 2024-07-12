use actix_web::{web, HttpRequest, Responder};
use actix_web_actors::ws;
use uuid::Uuid;

use crate::{db::DbPool, ws::WsChatSession};

pub async fn ws_index(
    req: HttpRequest,
    stream: web::Payload,
    db_pool: web::Data<DbPool>,
) -> impl Responder {
    let session = WsChatSession {
        id: Uuid::new_v4(),
        db_pool,
    };
    ws::start(session, &req, stream)
}
