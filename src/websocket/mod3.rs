// pub mod lobby;
// pub mod messages;
// pub mod server2;
// pub mod start_connection;
// pub mod wss;
use std::time::{Duration, Instant};

use crate::Error;
use actix::{
    fut,
    prelude::{Actor, Addr, Handler, Running, StreamHandler},
    ActorContext, ActorFutureExt, AsyncContext, ContextFutureSpawner, WrapFuture,
};

use actix_web::{web, web::Path, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use actix_web_actors::ws::Message::Text;
use log::kv::ToValue;
use serde_json::{error::Result as SerdeResult, from_str, json, to_string, Value};
use uuid::Uuid;
mod server;
pub use self::server::*;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(30);

pub struct WebSocketSession {
    room: Uuid,
    id: Uuid,
    hb: Instant,
    server_addr: Addr<Server>,
}

impl WebSocketSession {
    fn new(room: Uuid, server_addr: Addr<Server>) -> Self {
        Self {
            room,
            id: Uuid::new_v4(),
            hb: Instant::now(),
            server_addr,
        }
    }

    fn send_heartbeat(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                log::info!("Websocket Client heartbeat failed, disconnecting!");
                act.server_addr.do_send(Disconnect {
                    id: act.id,
                    room_id: act.room,
                });
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
                server_id: self.room,
                id: self.id,
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
    // fn stopping(&mut self, _: &mut Self::Context) -> Running {
    //     self.server_addr.do_send(Disconnect {
    //         id: self.id,
    //         room_id: self.room,
    //     });
    //     Running::Stop
    // }
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
            Ok(Text(s)) => {
                println!("{:?}",&s.to_string());
                self.server_addr.do_send(MessageToClient {
                    id:self.id,
                    msg: s.to_string(),
                    room_id: self.room
                    
                    // msg: serde_json::Value::String(s.to_string()),
                    // room_id: self.room.clone(),
                }
         
            )
            }
            Ok(ws::Message::Close(reason)) => {
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
    id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let res = ws::start(
        WebSocketSession::new(*id, server_addr.get_ref().clone()),
        &req,
        stream,
    )?;
println!("{:?}", res);
    Ok(res)
}
