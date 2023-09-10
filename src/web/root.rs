use crate::ctx::Context;
use askama_axum::IntoResponse;
use axum::routing::get;
use axum::Router;

pub fn routes() -> Router {
    Router::new().route("/", get(root))
}

async fn root(ctx: Option<Context>) -> axum::response::Response {
    match ctx {
        Some(_) => axum::response::Redirect::to("/home").into_response(),
        None => axum::response::Redirect::to("/login").into_response(),
    }
}
