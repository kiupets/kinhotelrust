use actix::Actor;
use actix::{Addr, Running, StreamHandler};
use actix_cors::Cors;
use actix_files::{Files, NamedFile};
use actix_web::{get, post, web, web::Data, web::Payload, Error, HttpRequest, HttpResponse};
use actix_web::{middleware, App, HttpServer, Responder, Result};
use actix_web_actors::ws;
use lobby::Lobby;
use start_connection::{send_statistics, start_connection as start_connection_route};
use std::env;
use websocket::lobby;
use websocket::messages;
use websocket::start_connection;

mod api;
mod models;
mod repository;
mod websocket;

use api::rented_api::{create_rented, get_rented};
use api::user_api::{create_user, get_user};
use repository::mongodb_repo::MongoRepo;

use websocket::server2::MyWebSocket;

async fn index() -> impl Responder {
    NamedFile::open_async("./build/index.html").await.unwrap()
}
async fn websocket(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(MyWebSocket::new(), &req, stream)
}
// async fn start_connection(
//     req: HttpRequest,
//     stream: Payload,
//     topic_name: web::Path<String>,
//     srv: Data<Addr<Lobby>>,
// ) -> Result<HttpResponse, Error> {
//     println!("client");

//     let topic_name = topic_name.into_inner();
//     let ws = WebSocketSession::new(topic_name, srv.get_ref().clone());
//     let resp = ws::start(ws, &req, stream)?;
//     println!("{:?}", resp);
//     Ok(resp)
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let chat_server = Lobby::default().start(); //create and spin up a lobby
    let HOST = env::var("HOST").expect("Host not set");
    let PORT = env::var("PORT").expect("Port not set");

    let db = MongoRepo::init();
    let db_data = Data::new(db);
    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .app_data(db_data.clone())
            // .service(web::resource("/ws").route(web::get().to(start_connection)))
            .service(start_connection_route)
            .data(chat_server.clone())
            .service(send_statistics)
            // .service(web::resource("/ws").route(web::get().to(websocket)))
            .service(create_user)
            .service(get_user)
            .service(create_rented)
            .service(get_rented)
            .service(Files::new("/", "./build"))
    })
    .workers(2)
    .bind(format!("{}:{}", HOST, PORT))?
    // .bind("127.0.0.1:8080")?
    .run()
    .await
}
