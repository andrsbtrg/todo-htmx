pub use crate::error::{Error, Result};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

mod error;
mod models;
mod templates;
mod web;

#[tokio::main]
async fn main() {
    let port = 3000;
    let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let socket = SocketAddr::new(ip, port);

    let app = web::app();
    println!("->> {:<12} - LISTENING on http://{}", "RUNNING", socket);
    axum::Server::bind(&socket)
        .serve(app.into_make_service())
        .await
        .unwrap()
}
