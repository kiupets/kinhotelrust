// use crate::lobby::Lobby;
// use crate::messages::BroadcastMessage;
use crate::websocket::{MessageToClient, Server};
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
    params: Json<Rented>,
    srv: Data<Addr<Server>>,
    // params: Json<Vec<StatisticRecord>>,
) -> HttpResponse {
    println!("rented api");
    let data = Rented {
        id: None,
        interval_rented_array: params.interval_rented_array.to_owned(),
    };
    if let Ok(data) = to_value(data.clone()) {
        let msg = MessageToClient::new("rented", data);
        srv.do_send(msg);
    }

    let rented_detail = db.create_rented(data).await;
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
    let rented_detail = db.get_rented(&id).await;
    match rented_detail {
        Ok(rented) => HttpResponse::Ok().json(rented),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/all")]
pub async fn get_all(db: Data<MongoRepo>) -> HttpResponse {
    println!("mammmmmmmmmmmmmmmmmmmmmmmmmm");
    let rents = db.get_all().await;
    println!("{:?}", rents);

    match rents {
        Ok(rents) => HttpResponse::Ok().json(rents),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
