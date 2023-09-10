use crate::ctx::Context;
use crate::templates::StartTemplate;
use askama_axum::IntoResponse;
use axum::routing::get;
use axum::Router;

pub fn routes() -> Router {
    Router::new().route("/home", get(home))
}

async fn home(ctx: Option<Context>) -> axum::response::Response {
    match ctx {
        Some(c) => {
            let template = StartTemplate {
                username: c.username(),
            };
            template.into_response()
        }
        None => axum::response::Redirect::to("/login").into_response(),
    }
}
