use std::collections::{HashMap, HashSet};

use actix::prelude::{Actor, Context, Handler, Message as ActixMessage, Recipient};
use serde::{Deserialize, Serialize};
use serde_json::{error::Result as SerdeResult, to_string, Value};
use uuid::Uuid;
#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(ActixMessage, Deserialize, Serialize)]
#[rtype(result = "()")]
pub struct MessageToClient {
    pub id: Uuid,
    pub msg: String,
    pub room_id: Uuid,
}

// impl MessageToClient {
//     // pub fn new(msg_type: &str, data: String, room_id: Uuid) -> Self {
//     //     Self {
//     //         id: Uuid::new(),
//     //         data,
//     //         room_id,
//     //     }
//     // }
// }
type Socket = Recipient<Message>;
pub struct Server {
    sessions: HashMap<Uuid, Socket>, //self id to self
    rooms: HashMap<Uuid, HashSet<Uuid>>,
}

impl Server {
    pub fn new() -> Self {
        Server {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
        }
    }

    fn send_message(&self, message: &str, id_to: &Uuid) {
        if let Some(socket_recipient) = self.sessions.get(id_to) {
            let _ = socket_recipient.do_send(Message(message.to_owned()));
            println!("{:?}", self.sessions.get(id_to))
        } else {
            println!("attempting to send message but couldn't find user id.");
        }
    }
    // fn send_message(&self, data: String, id_to: &Uuid) {
    //     match data {
    //         data => {
    //             println!("{:?}", data);
    //             for recipient in self.sessions.values() {
    //                 match recipient.try_send(Message(data.clone())) {
    //                     Err(err) => {
    //                         log::error!("Error sending client message: {:?}", err);
    //                     }
    //                     _ => {}
    //                 }
    //             }
    //         }
    //         err => {
    //             log::error!("Data did not convert to string {:?}", err);
    //         }
    //     }
    // }
}

impl Actor for Server {
    type Context = Context<Self>;
}

#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<Message>,
    pub id: Uuid,
    pub server_id: Uuid,
}
#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: Uuid,
    pub room_id: Uuid,
}

impl Handler<Connect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        self.rooms
            .entry(msg.server_id)
            .or_insert_with(HashSet::new)
            .insert(msg.id);

        self.rooms
            .get(&msg.server_id)
            .unwrap()
            .iter()
            .filter(|conn_id| *conn_id.to_owned() != msg.id)
            .for_each(|conn_id| self.send_message(&format!("{} just joined!", msg.id), conn_id));

        self.sessions.insert(msg.id, msg.addr);

        // self.send_message(&format!("your id is {}", msg.id), &msg.id);
    }
}

impl Handler<Disconnect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        if self.sessions.remove(&msg.id).is_some() {
            self.rooms
                .get(&msg.room_id)
                .unwrap()
                .iter()
                .filter(|conn_id| *conn_id.to_owned() != msg.id)
                .for_each(|user_id| {
                    self.send_message(&format!("{} disconnected.", &msg.id), user_id)
                });
            if let Some(lobby) = self.rooms.get_mut(&msg.room_id) {
                if lobby.len() > 1 {
                    lobby.remove(&msg.id);
                } else {
                    //only one in the lobby, remove it entirely
                    self.rooms.remove(&msg.room_id);
                }
            }
        }
    }
}

impl Handler<MessageToClient> for Server {
    type Result = ();

    fn handle(&mut self, msg: MessageToClient, _ctx: &mut Context<Self>) -> Self::Result {
        if msg.msg.starts_with("\\w") {
            if let Some(id_to) = msg.msg.split(' ').collect::<Vec<&str>>().get(1) {
                self.send_message(&msg.msg, &Uuid::parse_str(id_to).unwrap());
            }
        } else {
            self.rooms
                .get(&msg.room_id)
                .unwrap()
                .iter()
                .for_each(|client| self.send_message(&msg.msg, client));
        }
    }
}
