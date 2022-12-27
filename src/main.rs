mod lobby;
mod ws;
use lobby::Lobby;
mod messages;
mod start_connection;
use actix::Actor;
use start_connection::start_connection as start_connection_route;

use std::env;

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{web, web::Data, Error};
use actix_web::{App, HttpServer};
// use actix_web_actors::ws;

mod api;
mod models;
mod repository;
mod websocket;
use api::rented_api::{create_rented, get_all, get_rented, update_rented};
use api::user_api::{create_user, get_user};
use repository::mongodb_repo::MongoRepo;
use websocket::ws_index;
// mod settings;

// pub fn routes(cfg: &mut web::ServiceConfig) {
//     cfg.service(web::resource("/ws").route(web::get().to(ws_index)));
// }
#[actix_web::main]

async fn main() -> std::io::Result<()> {
    let server = websocket::Server::new().start();
    // let chat_server = Lobby::new().start();
    // let HOST = env::var("HOST").expect("Host not set");
    // let PORT = env::var("PORT").expect("Port not set");

    let db = MongoRepo::init().await;
    let db_data = Data::new(db);
    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            // .data(chat_server.clone())
            // .service(start_connection_route)
            .app_data(db_data.clone())
            // .route("/ws", web::get().to(start_connection_route))
            .data(server.clone())
            .service(create_user)
            .route("/ws", web::get().to(ws_index))
            .service(get_user)
            .service(create_rented)
            .service(get_rented)
            .service(update_rented)
            .service(get_all)
            .service(Files::new("/", "./build").index_file("index.html"))
    })
    .workers(2)
    // .bind(format!("{}:{}", HOST, PORT))?
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
