use actix::Actor;

// use actix_cors::Cors;
use actix_files::Files;
use actix_web::{web, web::Data, Error};
use actix_web::{App, HttpServer};
// use actix_web_actors::ws;

mod api;
mod models;
mod repository;
mod websocket;
use api::rented_api::{create_rented, get_all, get_rented};
use api::user_api::{create_user, get_user};
use repository::mongodb_repo::MongoRepo;
use websocket::ws_index;
mod settings;

// pub fn routes(cfg: &mut web::ServiceConfig) {
//     cfg.service(web::resource("/ws").route(web::get().to(ws_index)));
// }
#[actix_web::main]

async fn main() -> std::io::Result<()> {
    let server = websocket::Server::new().start();
    let settings = crate::settings::Settings::new().unwrap();
    let server_address = format!("{}:{}", settings.server_address, settings.server_port);
    // let HOST = env::var("HOST").expect("Host not set");
    // let PORT = env::var("PORT").expect("Port not set");

    let db = MongoRepo::init().await;
    let db_data = Data::new(db);
    HttpServer::new(move || {
        // let cors = Cors::permissive();

        App::new()
            // .wrap(cors)
            .app_data(db_data.clone())
            .data(server.clone())
            .service(create_user)
            .route("/ws", web::get().to(ws_index))
            .service(get_user)
            .service(create_rented)
            .service(get_rented)
            .service(get_all)
            .service(Files::new("/", "./build").index_file("index.html"))
    })
    // .workers(2)
    // .bind(format!("{}:{}", HOST, PORT))?
    .bind(server_address)?
    .run()
    .await
}
