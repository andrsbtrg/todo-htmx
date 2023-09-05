pub mod home;
pub mod login;
pub mod logout;
pub mod mw_auth;
pub mod profile;
pub mod register;
pub mod routes_static;
pub mod tickets;

use askama_axum::{IntoResponse, Response};
use axum::{middleware, Extension, Json, Router};
use sqlx::SqlitePool;
use tower_cookies::CookieManagerLayer;

use crate::{models::ModelController, Error};

use self::mw_auth::mw_async_resolver;

pub const AUTH_TOKEN: &str = "auth-token";

pub fn app(pool: SqlitePool) -> Router {
    let model_controller = ModelController::new(pool).unwrap();

    let routes_api = tickets::routes().route_layer(middleware::from_fn(mw_auth::mw_require_auth));

    Router::new()
        .merge(home::routes())
        .merge(login::routes())
        .merge(logout::routes())
        .merge(register::routes())
        .merge(profile::routes())
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

    let uuid = uuid::Uuid::new_v4();

    let service_error = res.extensions().get::<Error>();

    let client_status_error = service_error.map(|se| se.client_status_and_error());

    // If client error, build new response

    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = serde_json::json!({
                "error": {
                "type": client_error.as_ref(),
                "req_uuid": uuid.to_string(),
            }
            });
            println!("->> client_error_body: {client_error_body}");
            // Build the new response
            (*status_code, Json(client_error_body)).into_response()
        });

    println!("->> server log line - {uuid} - Error: {service_error:?}");

    println!();
    error_response.unwrap_or(res)
}
