use crate::{
    models::{ModelController, UserCreate},
    web, Result,
};
use askama_axum::IntoResponse;
use axum::{http::HeaderMap, routing::post, Extension, Form, Router};
use serde::Deserialize;
use tower_cookies::{Cookie, Cookies};

pub fn routes() -> Router {
    Router::new().route("/api/register", post(register_new))
}

async fn register_new(
    Extension(mc): Extension<ModelController>,
    cookies: Cookies,
    Form(payload): Form<RegisterPayload>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - register", "HANDLER");

    let hashed_pwd = pwhash::bcrypt::hash(payload.password).unwrap();

    let user_fc = UserCreate {
        username: payload.username,
        hashed_password: hashed_pwd,
    };

    let user_id = mc.register_new(user_fc).await?;

    let cookie_value = format!("user-{}.exp.sign", user_id);

    // FIXME: Implement real cookies
    let cookie = Cookie::build(web::AUTH_TOKEN, cookie_value)
        .path("/")
        .finish();
    cookies.add(cookie);

    let mut headers = HeaderMap::new();
    headers.insert("HX-Redirect", "/".parse().unwrap());

    Ok(headers)
}

#[derive(Debug, Deserialize)]
struct RegisterPayload {
    username: String,
    password: String,
}
