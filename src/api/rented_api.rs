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
use serde_json::to_value;
use uuid::Uuid;
#[post("/rented")]
pub async fn create_rented(
    db: Data<MongoRepo>,
    new_rented: Json<Rented>,
    srv: Data<Addr<Lobby>>,
) -> HttpResponse {
    let data = Rented {
        id: None,
        interval_rented_array: new_rented.interval_rented_array.to_owned(),
    };

    let my_uuid = Uuid::parse_str("60d1dfc8-4e0d-4f18-8d17-cbc838313d55");
    if let Ok(data) = to_value(data.clone()) {
        let msg = MessageToClient::new("rented", data);
        srv.do_send(msg);
    }
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
