pub mod home;
pub mod login;
pub mod routes_static;
pub mod tickets;

use askama_axum::Response;
use axum::{middleware, Extension, Router};
use sqlx::SqlitePool;
use tower_cookies::CookieManagerLayer;

use crate::models::ModelController;

pub const AUTH_TOKEN: &str = "auth-token";

pub fn app(pool: SqlitePool) -> Router {
    let model_controller = ModelController::new(pool).unwrap();
    Router::new()
        .merge(home::routes())
        .merge(login::routes())
        .merge(tickets::routes())
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .layer(Extension(model_controller))
        .fallback_service(routes_static::routes())
}
async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "MAPPER");

    println!();
    res
}
