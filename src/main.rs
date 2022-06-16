use actix_cors::Cors;
use actix_files::{Files, NamedFile};
use actix_web::{
    get, http::header, post, web, web::Data, App, HttpResponse, HttpServer, Responder, Result,
};
use serde::{Deserialize, Serialize};

mod api;
mod models;
mod repository;

use api::rented_api::{create_rented, get_rented};
use api::user_api::{create_user, get_user};

use repository::mongodb_repo::MongoRepo;

// #[get("/")]
// async fn hello() -> Result<NamedFile> {
//     Ok(NamedFile::open("./build/index.html")?)
// }

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }
// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().json("hi there")
// }
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init();
    let db_data = Data::new(db);
    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .app_data(db_data.clone())
            .service(create_user)
            .service(get_user)
            .service(create_rented)
            .service(get_rented)
            .service(Files::new("/", "./build"))
        // .service(hello)
        // .service(echo)
        // .route("/manual_hello", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
