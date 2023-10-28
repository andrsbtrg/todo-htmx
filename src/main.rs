mod configuration;
mod ctx;
mod error;
mod models;
mod templates;
mod web;

pub use crate::error::{Error, Result};
use sqlx::PgPool;
use std::net::{IpAddr, SocketAddr};

#[tokio::main]
async fn main() {
    let config = configuration::get_configuration().expect("Unable to load configurationfile.");

    let ip: IpAddr = config.addr.parse().unwrap();
    let socket = SocketAddr::new(ip, config.port);

    let db_url = config.postgres_connection_string();

    let connection_pool = PgPool::connect(&db_url)
        .await
        .expect("Failed to connect to Postgres.");

    let app = web::app(connection_pool);
    println!("->> {:<12} - LISTENING on http://{}", "RUNNING", socket);
    axum::Server::bind(&socket)
        .serve(app.into_make_service())
        .await
        .unwrap()
}
