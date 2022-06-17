use actix_cors::Cors;
use actix_files::{Files, NamedFile};
use actix_web::{web::Data, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};
use std::env;

mod api;
mod models;
mod repository;

use api::rented_api::{create_rented, get_rented};
use api::user_api::{create_user, get_user};

use repository::mongodb_repo::MongoRepo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let HOST = env::var("HOST").expect("Host not set");
    let PORT = env::var("PORT").expect("Port not set");
    // let port = env::var("PORT")
    //     .unwrap_or_else(|_| "3000".to_string())
    //     .parse::<u16>()
    //     .expect("fuck you");

    let db = MongoRepo::init();
    let db_data = Data::new(db);
    HttpServer::new(move || {
        // let cors = Cors::permissive();

        App::new()
            // .wrap(cors)
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
    .bind(format!("{}:{}", HOST, PORT))?
    .run()
    .await
}
