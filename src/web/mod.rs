pub mod home;
pub mod login;
pub mod mw_auth;
pub mod routes_static;
pub mod tickets;

use askama_axum::Response;
use axum::{middleware, Extension, Router};
use sqlx::SqlitePool;
use tower_cookies::CookieManagerLayer;

use crate::models::ModelController;

use self::mw_auth::mw_async_resolver;

pub const AUTH_TOKEN: &str = "auth-token";

pub fn app(pool: SqlitePool) -> Router {
    let model_controller = ModelController::new(pool).unwrap();

    let routes_api = tickets::routes().route_layer(middleware::from_fn(mw_auth::mw_require_auth));

    Router::new()
        .merge(home::routes())
        .merge(login::routes())
        .merge(routes_api)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
            model_controller.clone(),
            mw_async_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .layer(Extension(model_controller))
        .fallback_service(routes_static::routes())
}
async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "MAPPER");

    println!();
    res
}
