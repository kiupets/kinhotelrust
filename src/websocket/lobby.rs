use crate::messages::{BroadcastMessage, Connect, Disconnect, Message, MessageToClient, WsMessage};
use actix::prelude::{Actor, Context, Handler, Recipient};
use serde_json::to_string;
use serde_json::{error::Result as SerdeResult, Value};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;
type Socket = Recipient<WsMessage>;

pub struct Lobby {
    sessions: HashMap<Uuid, Socket>,       //self id to self
    rooms: HashMap<String, HashSet<Uuid>>, //room id  to list of users id
}

impl Default for Lobby {
    fn default() -> Lobby {
        Lobby {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
        }
    }
}

impl Lobby {
    fn send_message(&self, data: SerdeResult<String>) {
        match data {
            Ok(data) => {
                for recipient in self.sessions.values() {
                    match recipient.try_send(WsMessage(data.clone())) {
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

impl Actor for Lobby {
    type Context = Context<Self>;
}

impl Handler<Disconnect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        if self.sessions.remove(&msg.id).is_some() {
            // self.rooms
            //     .get(&msg.room_id)
            //     .unwrap()
            //     .iter()
            //     .filter(|conn_id| *conn_id.to_owned() != msg.id)
            //     .for_each(|user_id| {
            //         self.send_message(&format!("{} disconnected.", &msg.id))
            //     });
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

impl Handler<Connect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        self.rooms
            .entry(msg.lobby_id.clone())
            .or_insert_with(HashSet::new)
            .insert(msg.self_id);

        // self.rooms
        //     .get(&msg.lobby_id)
        //     .unwrap()
        //     .iter()
        //     .filter(|conn_id| *conn_id.to_owned() != msg.self_id)
        //     .for_each(|conn_id| {
        //         self.send_message(&format!("{} just joined!", msg.self_id), conn_id)
        //     });

        self.sessions.insert(msg.self_id, msg.addr);

        // self.send_message(&format!("your id is {}", msg.self_id), &msg.self_id);
    }
}

/*
impl Handler<ClientActorMessage> for Lobby {
    type Result = ();
    fn handle(&mut self, msg: ClientActorMessage, _ctx: &mut Context<Self>) -> Self::Result {
        if msg.msg.starts_with("\\w") {
            if let Some(id_to) = msg.msg.split(' ').collect::<Vec<&str>>().get(1) {
                self.send_message(&msg.msg, &Uuid::parse_str(id_to).unwrap());
            }
        } else {
            self.rooms.get(&msg.room_id).unwrap().iter().for_each(|client| self.send_message(&msg.msg, client));
        }
    }
}
*/

impl Handler<BroadcastMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: BroadcastMessage, _ctx: &mut Context<Self>) -> Self::Result {
        if let Some(_socket_recipient) = self.sessions.get(&msg.id) {
            // self.rooms.get(&msg.room_id).unwrap().iter()
            // .for_each(|_client| self.send_message(&to_string(&msg).unwrap(), _client));
        } else {
            println!("attempting to send message but couldn't find admin id.");
        }
    }
}
impl Handler<MessageToClient> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: MessageToClient, _: &mut Context<Self>) -> Self::Result {
        self.send_message(to_string(&msg));
    }
}