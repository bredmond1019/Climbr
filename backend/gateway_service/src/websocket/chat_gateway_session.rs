use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web_actors::ws;
use awc::Client;

use super::{
    chat_service_connection::{ChatServiceConnection, ClientMessage, Connect, Disconnect},
    ChatServiceMessage,
};

pub struct ChatGatewaySession {
    addr: Addr<ChatServiceConnection>,
    hb: Instant,
}

impl Actor for ChatGatewaySession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        self.addr.do_send(Connect {
            addr: ctx.address().recipient(),
        });
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.addr.do_send(Disconnect);
        Running::Stop
    }
}

impl Handler<ClientMessage> for ChatGatewaySession {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, ctx: &mut Self::Context) {
        // Handle the ChatServiceMessage here
        ctx.text(msg.0);
    }
}

impl ChatGatewaySession {
    pub fn new(addr: Addr<ChatServiceConnection>) -> Self {
        Self {
            addr,
            hb: Instant::now(),
        }
    }

    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(Duration::new(5, 0), |act, ctx| {
            if Instant::now().duration_since(act.hb) > Duration::new(10, 0) {
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatGatewaySession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                self.addr.do_send(ClientMessage(text.to_string()));
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}

// pub struct ChatGatewaySession {
//     client: Client,
//     chat_service_addr: Addr<ChatServiceConnection>,
// }

// #[derive(Message)]
// #[rtype(result = "()")]
// struct SetChatServiceAddr(Addr<ChatServiceConnection>);

// impl ChatGatewaySession {
//     pub fn new(chat_service_addr: Addr<ChatServiceConnection>) -> Self {
//         ChatGatewaySession {
//             client: Client::new(),
//             chat_service_addr: chat_service_addr,
//         }
//     }
// }

// impl Actor for ChatGatewaySession {
//     type Context = ws::WebsocketContext<Self>;

//     fn started(&mut self, ctx: &mut Self::Context) {
//         let chat_service_url = "ws://127.0.0.1:8083/ws/";
//         let client = self.client.clone();
//         let addr = ctx.address();

//         actix::spawn(async move {
//             let (response, connection) = client
//                 .ws(chat_service_url)
//                 .connect()
//                 .await
//                 .expect("Failed to connect to chat service");
//             println!("Connected to chat service: {:?}", response);

//             // let chat_connection = ChatServiceConnection::new(addr.clone());
//             // let chat_service_address = chat_connection.clone().start();

//             let chat_service_connect_addr = ChatServiceConnection::create(|ctx| {
//                 ChatServiceConnection::add_stream(connection, ctx);
//                 let chat_connection = ChatServiceConnection::new(addr.clone());
//                 chat_connection.clone().start();
//                 chat_connection
//             });

//             addr.do_send(SetChatServiceAddr(chat_service_connect_addr));
//         });
//     }
// }

// impl Handler<ChatServiceMessage> for ChatGatewaySession {
//     type Result = ();

//     fn handle(&mut self, msg: ChatServiceMessage, ctx: &mut Self::Context) {
//         ctx.text(msg.0);
//     }
// }

// impl Handler<SetChatServiceAddr> for ChatGatewaySession {
//     type Result = ();

//     fn handle(&mut self, msg: SetChatServiceAddr, _: &mut Self::Context) {
//         self.chat_service_addr = Some(msg.0);
//     }
// }

// impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatGatewaySession {
//     fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
//         if let Ok(ws::Message::Text(text)) = msg {
//             if let Some(ref addr) = self.chat_service_addr {
//                 addr.do_send(ChatServiceMessage(text.to_string()));
//             }
//         }
//     }
// }
