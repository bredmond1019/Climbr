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
    // query: web::Query<InitialMessage>,
) -> impl Responder {
    let query_string = req.query_string();
    info!("Query: {}", query_string);

    // let query_params: Vec<i32> = query.user_ids;
    // info!("Query params: {:?}", &query.user_ids);

    let initial_message: Result<InitialMessage, _> = serde_qs::from_str(query_string);
    match initial_message {
        Ok(initial_msg) => {
            let user_ids = vec![initial_msg.sender_id, initial_msg.receiver_id];

            let mut conn = pool.get().expect("Failed to get DB connection");
            let conversation = Conversation::find_or_create_conversation(&mut conn, user_ids)
                .expect("Failed to find or create conversation");

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
