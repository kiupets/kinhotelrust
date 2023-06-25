use actix::prelude::{Actor, Context, Handler, Message as ActixMessage, Recipient};
use env_logger::Env;
use kinhotelrust::configuration::get_configuration;
use kinhotelrust::repository::mongodb_repo::MongoRepo;
use kinhotelrust::startup::run;
use kinhotelrust::websocket::server::Server;
use kinhotelrust::websocket2::server_shit::Server2;
use sqlx::PgPool;
use std::net::TcpListener;
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    // let socket_server = Server::new().start();
    // let socket_server2 = Server2::new().start();
    let db = MongoRepo::init().await;
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to postgress");
    // We have removed the hard-coded `8000` - it's now coming from our settings!
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool, db)?.await
}
