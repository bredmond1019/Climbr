use actix::Addr;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use log::info;

use crate::chat::chat_session::{ChatSession, InitialMessage};
use crate::db::DbPool;
use crate::models::conversation::Conversation;

use crate::ChatServer;

pub async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    server: web::Data<Addr<ChatServer>>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    let query = req.query_string();
    info!("Query: {}", query);

    let initial_message: Result<InitialMessage, _> = serde_qs::from_str(query);
    match initial_message {
        Ok(initial_msg) => {
            let user_ids = initial_msg.user_ids;
            info!("User IDs: {:?}", user_ids);

            let mut conn = pool.get().unwrap();
            let conversation =
                Conversation::find_or_create_conversation(&mut conn, user_ids).unwrap();

            let chat_server_address = server.get_ref().clone();
            let session = ChatSession::new(chat_server_address, conversation, &mut conn);
            ws::start(session, &req, stream)
        }

        Err(err) => {
            info!("Error parsing query string: {:?}", err);
            Ok(HttpResponse::BadRequest().body("Invalid query string"))
        }
    }
}
