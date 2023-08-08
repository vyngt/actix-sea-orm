use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;

use std::env;
use std::io;
use std::sync::Mutex;

mod handlers;
mod routes;

use routes::general_routes;

#[actix_rt::main]
pub async fn main() -> io::Result<()> {
    dotenv().ok();

    let app = move || App::new().configure(general_routes);

    let host_name: String = env::var("HOST_NAME").unwrap_or(String::from("127.0.0.1"));
    let host_port = env::var("HOST_PORT").unwrap_or(String::from("8080"));

    HttpServer::new(app)
        .bind(format!("{host_name}:{host_port}"))
        .unwrap()
        .run()
        .await
}
