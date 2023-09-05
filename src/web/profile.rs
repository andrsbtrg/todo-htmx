use axum::{routing::get, Extension, Router};

use crate::{ctx::Context, models::ModelController, templates::ProfileTemplate};

pub fn routes() -> Router {
    Router::new().route("/profile", get(profile_show))
}

async fn profile_show(Extension(mc): Extension<ModelController>, ctx: Context) -> ProfileTemplate {
    ProfileTemplate {}
}
