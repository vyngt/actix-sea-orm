use sea_orm::{Database, DatabaseConnection};
use std::env;

pub async fn get_db_connection() -> DatabaseConnection {
    let db_host = env::var("DB_HOST").expect("DB: DB_HOST is not set");
    let db_port = env::var("DB_PORT").expect("DB: DB_PORT is not set");
    let db_name = env::var("DB_NAME").expect("DB: DB_NAME is not set");
    let db_user = env::var("DB_USER").expect("DB: DB_USER is not set");
    let db_password = env::var("DB_PASSWORD").expect("DB: DB_PASSWORD is not set");

    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        db_user, db_password, db_host, db_port, db_name
    );

    let db = Database::connect(database_url).await;
    let connection = db.expect("DB Connection Problem...");
    connection
}
