use crate::{templates::StartTemplate, web::AUTH_TOKEN};
use askama_axum::IntoResponse;
use axum::routing::get;
use axum::Router;
use tower_cookies::Cookies;

pub fn routes() -> Router {
    Router::new().route("/", get(home))
}

async fn home(cookies: Cookies) -> axum::response::Response {
    if let Some(user_cookie) = cookies.get(AUTH_TOKEN) {
        let template = StartTemplate {
            username: user_cookie.value().to_owned(),
        };
        template.into_response()
    } else {
        axum::response::Redirect::to("/login").into_response()
    }
}
