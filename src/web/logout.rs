use crate::web::AUTH_TOKEN;
use crate::Result;
use axum::{http::HeaderMap, routing::get, Router};
use tower_cookies::{Cookie, Cookies};

pub fn routes() -> Router {
    Router::new().route("/logout", get(logout))
}

async fn logout(cookies: Cookies) -> Result<impl askama_axum::IntoResponse> {
    let mut headers = HeaderMap::new();
    cookies.remove(Cookie::named(AUTH_TOKEN));

    headers.insert("HX-Redirect", "/".parse().unwrap());

    Ok(headers)
}
