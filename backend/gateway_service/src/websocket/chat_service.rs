use actix::prelude::*;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use awc::Client;
use futures_util::stream::StreamExt;

use std::str;

struct ChatGatewaySession {
    client: Client,
}

impl ChatGatewaySession {
    fn new() -> Self {
        ChatGatewaySession {
            client: Client::new(),
        }
    }
}

impl Actor for ChatGatewaySession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let chat_service_url = "ws://127.0.0.1:8083/ws/";
        let client = self.client.clone();
        let addr = ctx.address();

        actix::spawn(async move {
            let (response, mut connection) = client
                .ws(chat_service_url)
                .connect()
                .await
                .expect("Failed to connect to chat service");
            println!("Connected to chat service: {:?}", response);

            while let Some(msg) = connection.next().await {
                if let Ok(ws::Frame::Text(txt)) = msg {
                    let text_string = str::from_utf8(&txt).expect("Invalid UTF-8");
                    let text_string = text_string.to_string();
                    addr.do_send(ChatServiceMessage(text_string));
                }
            }
        });
    }
}

#[derive(Message)]
#[rtype(result = "()")]
struct ChatServiceMessage(String);

impl Handler<ChatServiceMessage> for ChatGatewaySession {
    type Result = ();

    fn handle(&mut self, msg: ChatServiceMessage, ctx: &mut Self::Context) {
        ctx.text(bytestring::ByteString::from(msg.0));
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatGatewaySession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(ws::Message::Text(text)) = msg {
            ctx.text(text.clone());
        }
    }
}

pub async fn websocket_handler(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    ws::start(ChatGatewaySession::new(), &req, stream)
}
