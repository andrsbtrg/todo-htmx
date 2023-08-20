mod configuration;
mod error;
mod models;
mod templates;
mod web;

pub use crate::error::{Error, Result};
use sqlx::SqlitePool;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[tokio::main]
async fn main() {
    let config = configuration::get_configuration().expect("Unable to load configurationfile.");

    let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let socket = SocketAddr::new(ip, config.application_port);

    let db_url = config.db_connection_string();
    config.db_check().await;

    let db = SqlitePool::connect(&db_url).await.unwrap();

    let app = web::app(db);
    println!("->> {:<12} - LISTENING on http://{}", "RUNNING", socket);
    axum::Server::bind(&socket)
        .serve(app.into_make_service())
        .await
        .unwrap()
}
