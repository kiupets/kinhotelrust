use std::collections::HashMap;

use actix::prelude::{Actor, Context, Handler, Message as ActixMessage, Recipient};
use serde::{Deserialize, Serialize};
use serde_json::{error::Result as SerdeResult, to_string, Value};
use std::time::{Duration, Instant};

use actix::{
    fut,
    prelude::{Actor, Addr, Handler, StreamHandler},
    ActorContext, ActorFutureExt, AsyncContext, ContextFutureSpawner, WrapFuture,
};
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use log::kv::ToValue;
use serde_json::{error::Result as SerdeResult, from_str, json, to_string, Value};
use uuid::Uuid;

use crate::Error;
use actix_web_actors::ws::Message::Text;
mod server;
// pub use self::server::*;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(30);
#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(ActixMessage, Deserialize, Serialize)]
#[rtype(result = "()")]
pub struct MessageToClient {
    pub msg_type: String,
    pub data: Value,
}

impl MessageToClient {
    pub fn new(msg_type: &str, data: Value) -> Self {
        Self {
            msg_type: msg_type.to_string(),
            data,
        }
    }
}

pub struct Server {
    sessions: HashMap<String, Recipient<Message>>,
}

impl Server {
    pub fn new() -> Self {
        Server {
            sessions: HashMap::new(),
        }
    }

    fn send_message(&self, data: SerdeResult<String>) {
        match data {
            Ok(data) => {
                for recipient in self.sessions.values() {
                    match recipient.try_send(Message(data.clone())) {
                        Err(err) => {
                            log::error!("Error sending client message: {:?}", err);
                        }
                        _ => {}
                    }
                }
            }
            Err(err) => {
                log::error!("Data did not convert to string {:?}", err);
            }
        }
    }
}

impl Actor for Server {
    type Context = Context<Self>;
}

#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<Message>,
    pub id: String,
}

impl Handler<Connect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) {
        self.sessions.insert(msg.id.clone(), msg.addr);
        println!("{:?}", msg.id);
    }
}

#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: String,
}

impl Handler<Disconnect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        self.sessions.remove(&msg.id);
    }
}

impl Handler<MessageToClient> for Server {
    type Result = ();

    fn handle(&mut self, msg: MessageToClient, _: &mut Context<Self>) -> Self::Result {
        self.send_message(to_string(&msg));
    }
}

// pub mod lobby;
// pub mod messages;
// pub mod server2;
// pub mod start_connection;
// pub mod wss;

pub struct WebSocketSession {
    // room: String,
    id: String,
    hb: Instant,
    server_addr: Addr<Server>,
}

impl WebSocketSession {
    fn new(server_addr: Addr<Server>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            hb: Instant::now(),
            server_addr,
        }
    }

    fn send_heartbeat(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                log::info!("Websocket Client heartbeat failed, disconnecting!");
                act.server_addr.do_send(Disconnect { id: act.id.clone() });
                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }
            ctx.ping(b"");
        });
    }
}

impl Actor for WebSocketSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.send_heartbeat(ctx);

        let session_addr = ctx.address();
        self.server_addr
            .send(Connect {
                addr: session_addr.recipient(),
                id: self.id.clone(),
            })
            .into_actor(self)
            .then(|res, _act, ctx| {
                match res {
                    Ok(_res) => {}
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }
}

impl Handler<Message> for WebSocketSession {
    type Result = ();

    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(Text(s)) => self.server_addr.do_send(MessageToClient {
                msg_type: "renterd".to_string(),
                data:from_str(&s.to_string()).unwrap()
                // id: self.id,
                 // msg: serde_json::Value::String(s.to_string()),
                // room_id: self.room.clone(),
            }),
            Ok(ws::Message::Close(reason)) => {
                log::info!("closed ws session");
                self.server_addr.do_send(Disconnect {
                    id: self.id.clone(),
                });
                ctx.close(reason);
                ctx.stop();
            }
            Err(err) => {
                log::warn!("Error handling msg: {:?}", err);
                ctx.stop()
            }
            _ => ctx.stop(),
        }
    }
}

pub async fn ws_index(
    req: HttpRequest,
    stream: web::Payload,
    server_addr: web::Data<Addr<Server>>,
) -> Result<HttpResponse, Error> {
    let res = ws::start(
        WebSocketSession::new(server_addr.get_ref().clone()),
        &req,
        stream,
    )?;

    Ok(res)
}
