use actix::prelude::{Message as ActixMessage, Recipient};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);

#[derive(ActixMessage)]
#[rtype(result = "()")]
// pub struct Connect {
//     pub addr: Recipient<WsMessage>,
//     pub lobby_id: String,
//     pub self_id: Uuid,
// }
pub struct Connect {
    pub addr: Recipient<Message>,
    pub id: String,
}

#[derive(ActixMessage)]
#[rtype(result = "()")]
// pub struct Disconnect {
//     pub id: Uuid,
//     pub room_id: String,
// }
pub struct Disconnect {
    pub id: String,
}
/*
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientActorMessage {
    pub id: Uuid,
    pub msg: String,
    pub room_id: String
}
 */

#[derive(ActixMessage, Deserialize, Serialize, Clone)]
#[rtype(result = "()")]
pub struct BroadcastMessage {
    pub id: Uuid,
    pub msg: Value,
    pub room_id: String,
}

impl BroadcastMessage {
    pub fn new(id: Uuid, data: Value, r_id: String) -> Self {
        Self {
            id,
            msg: data,
            room_id: r_id,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StatisticRecord {
    pub product_line: Option<String>,
    pub product_line_id: i32,
    pub total_trx: Option<i64>,
    pub total_amount: Option<f64>,
    pub currency: Option<String>,
    pub group_status: Option<String>,
}

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
