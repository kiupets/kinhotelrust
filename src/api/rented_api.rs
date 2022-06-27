use crate::lobby::Lobby;
use crate::messages::BroadcastMessage;
use crate::messages::MessageToClient;
use crate::{models::rented_model::Rented, repository::mongodb_repo::MongoRepo};
use actix::Addr;
use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse,
};
use serde_json::json;

// use serde_json::json;
use serde_json::to_value;
use uuid::Uuid;
#[post("/rented")]
pub async fn create_rented(
    db: Data<MongoRepo>,
    new_rented: Json<Rented>,
    srv: Data<Addr<Lobby>>,
    // params: Json<Vec<StatisticRecord>>,
) -> HttpResponse {
    let data = Rented {
        id: None,
        interval_rented_array: new_rented.interval_rented_array.to_owned(),
    };
    let _msg = new_rented.into_inner();

    let msg = BroadcastMessage {
        id: Uuid::parse_str("470bb217-ffa7-43d8-a0cc-b3d30421d1a9").unwrap(),
        msg: json!(_msg),
        room_id: "dailyCollection".to_string(),
    };
    srv.do_send(msg);

    let rented_detail = db.create_rented(data);
    match rented_detail {
        Ok(rented) => HttpResponse::Ok().json(rented),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/rented/{id}")]
pub async fn get_rented(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let rented_detail = db.get_rented(&id);
    match rented_detail {
        Ok(rented) => HttpResponse::Ok().json(rented),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
