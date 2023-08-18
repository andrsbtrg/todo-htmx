use crate::{templates, web, Error, Result};
use askama_axum::IntoResponse;
use axum::{http::HeaderMap, routing::post, Form, Router};
use serde::Deserialize;
use tower_cookies::{Cookie, Cookies};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}
async fn api_login(
    cookies: Cookies,
    Form(payload): Form<LoginPayload>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - api_login", "HANDLER");

    if payload.username != "dev" || payload.password != "password" {
        return Err(Error::LoginFail);
    }

    // FIXME: Implement real cookies
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

    // let body = Json(serde_json::json!({
    //     "result": {
    //     "success": true
    // }
    // }));
    // Ok(body)
    let mut headers = HeaderMap::new();
    headers.insert("HX-Redirect", "/".parse().unwrap());

    // let user_header = templates::HeaderUserTemplate {
    //     name: payload.username,
    // };

    Ok(headers)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}
