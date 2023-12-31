use crate::{models::ModelController, web, Error, Result};
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
        .route("/api/login", post(api_login))
        .route("/api/login", get(validate_username))
}
async fn api_login(
    Extension(mc): Extension<ModelController>,
    cookies: Cookies,
    Form(payload): Form<LoginPayload>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - api_login", "HANDLER");

    let mut headers = HeaderMap::new();

    match mc.get_pwd(&payload.username).await {
        Some(pwd) => {
            // FIXME: Implement real cookies
            if pwhash::bcrypt::verify(payload.password, &pwd) {
                let user_id: i32 = mc.get_user_id(&payload.username).await.unwrap();
                let cookie = Cookie::build(web::AUTH_TOKEN, format!("user-{user_id}.exp.sign"))
                    .path("/")
                    .finish();
                cookies.add(cookie);

                // builder.header("HX-Redirect", "/home");
                headers.insert("HX-Redirect", "/home".parse().unwrap());
            } else {
                // response = builder.body("Wrong password").unwrap();
                println!("Wrong password");
                return Ok((headers, "Wrong password"));
            }
        }

        None => return Err(Error::LoginFail),
    }

    Ok((headers, ""))
}
async fn validate_username(
    Extension(mc): Extension<ModelController>,
    Query(payload): Query<web::register::UserValidationPayload>,
) -> impl IntoResponse {
    println!("->> {:<12} - validate-username", "HANDLER");
    if payload.username.is_empty() {
        return "".into_response();
    }
    match mc.username_exists(&payload.username).await {
        true => "Looks fine".into_response(),
        false => "".into_response(),
    }
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}
