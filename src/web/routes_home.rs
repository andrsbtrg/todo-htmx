use std::sync::{Arc, Mutex};

use crate::{
    templates::{ItemsTemplate, StartTemplate},
    web::AUTH_TOKEN,
};
use askama_axum::IntoResponse;
use axum::{extract::State, routing::get, Router};
use tower_cookies::Cookies;

#[derive(Clone)]
struct AppState {
    counter: Arc<Mutex<i32>>,
}

pub fn routes() -> Router {
    let state = AppState {
        counter: Arc::new(Mutex::new(0)),
    };

    Router::new()
        .route("/items", get(get_items))
        .route("/", get(home))
        .with_state(state)
}

async fn home(cookies: Cookies) -> axum::response::Response {
    if let Some(user_cookie) = cookies.get(AUTH_TOKEN) {
        println!("Cookie!");
        let template = StartTemplate {
            username: user_cookie.value().to_owned(),
        };
        template.into_response()
    } else {
        axum::response::Redirect::to("/login").into_response()
    }
}

// TODO! delete me
async fn get_items(State(state): State<AppState>) -> ItemsTemplate {
    println!("->> {:<12} - get_items", "HANDLER");
    let mut counter = state.counter.lock().expect("state has been poisoned.");
    *counter += 1;
    let template = ItemsTemplate { counter: *counter };
    template
}
