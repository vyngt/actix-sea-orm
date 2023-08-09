use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;

use std::env;
use std::io;

mod database;
mod handlers;
mod models;
mod routes;
mod state;

use database::get_db_connection;
use routes::{course_routes, general_routes};
use state::AppState;

#[actix_rt::main]
pub async fn main() -> io::Result<()> {
    dotenv().ok();

    let db = get_db_connection().await;

    let app_state = web::Data::new(AppState { db });

    let app = move || {
        App::new()
            .app_data(app_state.clone())
            .configure(general_routes)
            .configure(course_routes)
    };

    let host_name: String = env::var("HOST_NAME").unwrap_or(String::from("127.0.0.1"));
    let host_port = env::var("HOST_PORT").unwrap_or(String::from("8080"));

    HttpServer::new(app)
        .bind(format!("{host_name}:{host_port}"))
        .unwrap()
        .run()
        .await
}
