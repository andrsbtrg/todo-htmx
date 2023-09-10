use axum::{routing::get, Extension, Router};

use crate::{ctx::Context, models::ModelController, templates::ProfileTemplate};

pub fn routes() -> Router {
    Router::new().route("/profile", get(profile_show))
}

async fn profile_show(Extension(_mc): Extension<ModelController>, ctx: Context) -> ProfileTemplate {
    ProfileTemplate {
        username: ctx.username().to_string(),
        user_id: ctx.user_id(),
    }
}
