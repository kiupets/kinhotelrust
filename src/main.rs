<<<<<<< HEAD
use kinhotelrust::websocket;
// use crate::websocket2;
use actix::prelude::{Actor, Context, Handler, Message as ActixMessage, Recipient};
use env_logger::Env;
use kinhotelrust::configuration::get_configuration;
// use kinhotelrust::issue_delivery_worker::run_worker_until_stopped;
use kinhotelrust::repository::mongodb_repo::MongoRepo;
use kinhotelrust::startup::run;
use kinhotelrust::startup::Application;
use kinhotelrust::websocket::server::Server;
use kinhotelrust::websocket2::server_shit::Server2;
use sqlx::PgPool;
use std::fmt::{Debug, Display};
use std::net::TcpListener;
use tokio::task::JoinError;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    // let websocket_task = websocket::server::Server::new().start();
    // let socket_server2 = Server2::new().start();
    // let websocket_task = tokio::spawn(websocket::server::Server::new().start());
    // let db = MongoRepo::init().await;
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration.clone()).await?;
    let application_task = tokio::spawn(application.run_until_stopped());
    // let application_task = tokio::spawn(application.run_until_stopped());
    // let worker_task = tokio::spawn(run_worker_until_stopped(configuration));
    // tokio::spawn(async move {
    //     let _ = websocket::server::Server::new().start();
    // });
    tokio::select! {
        o = application_task => report_exit("API", o),
        // o = worker_task =>  report_exit("Background worker", o),
    };
    // let connection_pool = PgPool::connect(&configuration.database.connection_string())
    // .await
    // .expect("Failed to connect to postgress");
    // We have removed the hard-coded `8000` - it's now coming from our settings!
    // let address = format!("127.0.0.1:{}", configuration.application_port);
    // let listener = TcpListener::bind(address)?;
    // run(listener, connection_pool, db, websocket_task)?.await
    Ok(())
}
fn report_exit(task_name: &str, outcome: Result<Result<(), impl Debug + Display>, JoinError>) {
    match outcome {
        Ok(Ok(())) => {
            tracing::info!("{} has exited", task_name)
        }
        Ok(Err(e)) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{} failed",
                task_name
            )
        }
        Err(e) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{}' task failed to complete",
                task_name
            )
        }
    }
=======
use actix::Actor;

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
mod websocket2;

use api::login_api::login;
use api::rented_api::{create_rented, get_all, get_rented, update_rented};
use api::user_api::{create_user, get_user};
use repository::mongodb_repo::MongoRepo;
use websocket::ws_index;
use websocket2::ws_index_drag;
// mod settings;

// pub fn routes(cfg: &mut web::ServiceConfig) {
//     cfg.service(web::resource("/ws").route(web::get().to(ws_index)));
// }
#[actix_web::main]

async fn main() -> std::io::Result<()> {
    let server = websocket::Server::new().start();
    let server2 = websocket2::Server2::new().start();

    let HOST = env::var("HOST").expect("Host not set");
    let PORT = env::var("PORT").expect("Port not set");

    let db = MongoRepo::init().await;
    let db_data = Data::new(db);
    HttpServer::new(move || {
        // let cors = Cors::permissive();

        App::new()
            // .wrap(cors)
            // .data(chat_server.clone())
            // .service(start_connection_route)
            .app_data(db_data.clone())
            // .route("/ws", web::get().to(start_connection_route))
            .data(server.clone())
            .data(server2.clone())
            .service(create_user)
            .route("/ws", web::get().to(ws_index))
            .route("/wss", web::get().to(ws_index_drag))
            .service(get_user)
            .service(create_rented)
            .service(get_rented)
            .service(update_rented)
            .service(get_all)
            .service(login)
            .service(Files::new("/", "./build").index_file("index.html"))
    })
    .workers(2)
    .bind(format!("{}:{}", HOST, PORT))?
    // .bind("127.0.0.1:8000")?
    .run()
    .await
>>>>>>> 9219adfb90170b831833b9dbf9f07ab274c3a509
}
