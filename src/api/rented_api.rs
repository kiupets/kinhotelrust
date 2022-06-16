use crate::{models::rented_model::Rented, repository::mongodb_repo::MongoRepo};
use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse,
};

#[post("/rented")]
pub async fn create_rented(db: Data<MongoRepo>, new_rented: Json<Rented>) -> HttpResponse {
    let data = Rented {
        id: None,
        interval_rented_array: new_rented.interval_rented_array.to_owned(),
    };
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
