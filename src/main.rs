use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

pub use crate::error::{Error, Result};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

mod error;
mod models;
mod templates;
mod web;
const DB_URL: &str = "sqlite://sqlite.db";

#[tokio::main]
async fn main() {
    let port = 3000;
    let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let socket = SocketAddr::new(ip, port);

    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    let app = web::app(db);
    println!("->> {:<12} - LISTENING on http://{}", "RUNNING", socket);
    axum::Server::bind(&socket)
        .serve(app.into_make_service())
        .await
        .unwrap()
}
