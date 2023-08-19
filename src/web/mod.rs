use askama_axum::Response;
use axum::{middleware, Router};
use tower_cookies::CookieManagerLayer;

pub mod routes_home;
pub mod routes_login;
pub mod routes_static;

pub const AUTH_TOKEN: &str = "auth-token";

pub fn app() -> Router {
    Router::new()
        .merge(routes_home::routes())
        .merge(routes_login::routes())
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static::routes())
}
async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "MAPPER");

    println!();
    res
}
