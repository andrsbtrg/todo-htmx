use crate::{models::ModelController, web, Error, Result};
use askama_axum::IntoResponse;
use axum::{http::HeaderMap, routing::post, Extension, Form, Router};
use serde::Deserialize;
use tower_cookies::{Cookie, Cookies};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
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
                let user_id: u32 = mc.get_user_id(&payload.username).await.unwrap();
                let cookie = Cookie::build(web::AUTH_TOKEN, format!("user-{user_id}.exp.sign"))
                    .path("/")
                    .finish();
                cookies.add(cookie);

                headers.insert("HX-Redirect", "/".parse().unwrap());
            }
        }
        None => return Err(Error::LoginFail),
    }
    // if payload.username != "dev" || payload.password != "password" {
    //     return Err(Error::LoginFail);
    // }

    // let user_header = templates::HeaderUserTemplate {
    //     name: payload.username,
    // }

    Ok(headers)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}
