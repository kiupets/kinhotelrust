use actix::Actor;

use actix::prelude::Addr;
use actix_cors::Cors;
use actix_files::Files;
use actix_web::middleware::Logger;
use actix_web::{web, web::Data, Error};
use actix_web::{App, HttpResponse, HttpServer};
use std::env;
use std::ops::Add;
// use actix_web_actors::ws;

// mod api;
// use crate::pos_routes;
use crate::routes::health_check::*;
use crate::routes::subscriptions::*;
// mod websocket;
// mod websocket2;
use crate::websocket;
use crate::websocket2;

// mod configuration;
use crate::api::login_api::login;
use crate::api::rented_api::{create_rented, get_all, get_rented, update_rented};
use crate::api::user_api::{create_user, get_user};
use crate::repository::mongodb_repo::MongoRepo;

// mod settings;
use actix_web::dev::Server;
use sqlx::PgConnection;
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
    db: MongoRepo,
) -> Result<Server, std::io::Error> {
    // let HOST = env::var("HOST").expect("Host not set");
    // let PORT = env::var("PORT").expect("Port not set");
    let server = websocket::Server::new().start();
    let server2 = websocket2::Server2::new().start();
    // let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    let db_pool = Data::new(db_pool);
    let server = HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
            .app_data(db_data.clone())
            // .app_data(socket_server.clone())
            // .app_data(socket_server2.clone())
            .service(create_user)
            // .route("/ws", web::get().to(websocket::ws_index))
            // .route("/wss", web::get().to(ws_index_drag))
            .service(get_user)
            .service(create_rented)
            .service(get_rented)
            .service(update_rented)
            .service(get_all)
            // .service(login)
            .service(Files::new("/", "./build").index_file("index.html"))
    })
    // .workers(4)
    // .bind(format!("{}:{}", HOST, PORT))?
    .listen(listener)?
    .run();
    Ok(server)
}
