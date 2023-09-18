use crate::{
    models::{ModelController, UserCreate},
    web, Result,
};
use askama_axum::IntoResponse;
use axum::{
    extract::Query,
    http::HeaderMap,
    routing::{get, post},
    Extension, Form, Router,
};
use serde::Deserialize;
use tower_cookies::{Cookie, Cookies};

pub fn routes() -> Router {
    Router::new()
        .route("/api/register", post(register_new))
        .route("/api/register", get(check_username))
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
    headers.insert("HX-Redirect", "/home".parse().unwrap());

    Ok(headers)
}

#[derive(Deserialize)]
pub struct UserValidationPayload {
    pub username: String,
}
async fn check_username(
    Extension(mc): Extension<ModelController>,
    Query(payload): Query<UserValidationPayload>,
) -> impl IntoResponse {
    println!("->> {:<12} - validate-username", "HANDLER");
    if payload.username.is_empty() {
        return "".into_response();
    }
    match mc.username_exists(&payload.username).await {
        true => "Username already taken".into_response(),
        false => "".into_response(),
    }
}

#[derive(Debug, Deserialize)]
struct RegisterPayload {
    username: String,
    password: String,
}
