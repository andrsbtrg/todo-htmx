use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::{Arc, Mutex},
};

mod templates;
use templates::ItemsTemplate;
use tower_http::services::ServeDir;

use axum::{
    extract::State,
    routing::{get, get_service},
    Router,
};

#[derive(Clone)]
struct AppState {
    counter: Arc<Mutex<i32>>,
}
#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(routes_app())
        .fallback_service(routes_static());

    let port = 3000;
    let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let socket = SocketAddr::new(ip, port);
    println!("->> LISTENING on http://{}", socket);
    axum::Server::bind(&socket)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn routes_app() -> Router {
    let state = AppState {
        counter: Arc::new(Mutex::new(0)),
    };

    Router::new()
        .route("/items", get(get_items))
        .with_state(state)
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./static")))
}

async fn get_items(State(state): State<AppState>) -> ItemsTemplate {
    println!("->> HANDLER get_items");
    let mut counter = state.counter.lock().expect("state has been poisoned.");
    *counter += 1;
    let template = ItemsTemplate { counter: *counter };
    template
}
