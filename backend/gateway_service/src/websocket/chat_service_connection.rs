use std::sync::Arc;

use actix::prelude::*;
use actix_codec::Framed;
use actix_web_actors::ws;
use awc::{ws::Codec, BoxedSocket, Client};
use futures_util::StreamExt;
use futures_util::{lock::Mutex, SinkExt, TryStreamExt};

// use std::str;

pub struct ChatServiceConnection {
    addr: String,
    client: Client,
    ws_sink: Option<Arc<Mutex<Option<Framed<BoxedSocket, Codec>>>>>,
}

impl ChatServiceConnection {
    pub fn new(addr: String) -> Self {
        ChatServiceConnection {
            addr,
            client: Client::new(),
            ws_sink: None,
        }
    }
}

impl Actor for ChatServiceConnection {
    type Context = Context<Self>;
}

pub struct Connect {
    pub addr: Recipient<ClientMessage>,
}

impl Message for Connect {
    type Result = ();
}

impl Handler<Connect> for ChatServiceConnection {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Self::Context) {
        let chat_addr = self.addr.clone();
        let client_addr = msg.addr;
        let client = self.client.clone();
        let ws_sink = Arc::new(Mutex::new(None));
        self.ws_sink = Some(ws_sink.clone());

        actix::spawn(async move {
            let (response, mut connection) = client
                .ws(chat_addr)
                .connect()
                .await
                .expect("Failed to connect to chat service");

            println!("Connected to chat service: {:?}", response);

            let (mut ws_sink, mut ws_stream) = connection.split();

            // *ws_sink.lock().await = Some(connection);
            // *ws_sink.lock().await = Some(ws_sink.clone());
            // if let Some(ws_sink) = &self.ws_sink {
            //     let message = msg.clone();
            //     let ws_sink = ws_sink.clone();
            //     actix::spawn(async move {
            //         if let Some(connection) = ws_sink.lock().await.as_mut() {
            //             let _ = connection.send(ws::Message::Text(message.into())).await;
            //         }
            //     });
            // }

            while let Ok(msg) = connection.try_next().await {
                if let Some(msg) = msg {
                    if let ws::Frame::Text(text) = msg {
                        let _ = client_addr
                            .do_send(ClientMessage(String::from_utf8_lossy(&text).to_string()));
                    }
                }
            }
        });
    }
}

pub struct Disconnect;

impl Message for Disconnect {
    type Result = ();
}

impl Handler<Disconnect> for ChatServiceConnection {
    type Result = ();

    fn handle(&mut self, _: Disconnect, _: &mut Self::Context) {
        // Handle disconnect logic here
    }
}

pub struct ClientMessage(pub String);

impl Message for ClientMessage {
    type Result = ();
}

impl Handler<ClientMessage> for ChatServiceConnection {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Self::Context) {
        println!("ClientMessage: {:?}", msg.0);
        if let Some(ws_sink) = &self.ws_sink {
            let message = msg.0.clone();
            let ws_sink = ws_sink.clone();
            actix::spawn(async move {
                if let Some(connection) = ws_sink.lock().await.as_mut() {
                    let _ = connection.send(ws::Message::Text(message.into())).await;
                }
            });
        }
    }
}

// #[derive(Clone)]
// pub struct ChatServiceConnection {
//     pub gateway_addr: Option<Addr<ChatGatewaySession>>,
//     pub chat_service_addr: String,
// }

// // impl ChatServiceConnection {
// //     pub fn new(gateway_addr: Addr<ChatGatewaySession>) -> Self {
// //         ChatServiceConnection { gateway_addr }
// //     }
// // }
// impl ChatServiceConnection {
//     pub fn new(chat_service_addr: String) -> Self {
//         ChatServiceConnection {
//             chat_service_addr,
//             gateway_addr: None,
//         }
//     }
// }

// impl Actor for ChatServiceConnection {
//     type Context = Context<Self>;
//     // type Context = ws::WebsocketContext<Self>;

//     fn started(&mut self, _: &mut Self::Context) {
//         println!("ChatServiceConnect started");
//     }

//     fn stopped(&mut self, _: &mut Self::Context) {
//         println!("ChatServiceConnect stopped");
//     }
// }

// impl StreamHandler<Result<ws::Frame, ws::ProtocolError>> for ChatServiceConnection {
//     fn handle(&mut self, msg: Result<ws::Frame, ws::ProtocolError>, ctx: &mut Self::Context) {
//         if let Ok(ws::Frame::Text(txt)) = msg {
//             if let Ok(text) = str::from_utf8(txt.as_ref()) {
//                 self.gateway_addr
//                     .do_send(ChatServiceMessage(text.to_string()));
//             }
//         }
//     }
// }

// impl Handler<ChatServiceMessage> for ChatServiceConnection {
//     type Result = ();

//     fn handle(&mut self, msg: ChatServiceMessage, ctx: &mut Self::Context) {
//         if let frame = awc::ws::Frame::Text(msg.0.into()) {
//             ctx.send(frame);
//         }
//     }
// }
