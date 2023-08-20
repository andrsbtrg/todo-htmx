use axum::{
    routing::{get, post},
    Extension, Form, Router,
};
use serde::Deserialize;

use crate::{
    models::{ModelController, TicketCreate},
    templates::{TicketTemplate, TicketsTemplate},
};

pub fn routes() -> Router {
    Router::new()
        .route("/tickets", get(get_tickets))
        .route("/tickets", post(create_ticket))
}

async fn get_tickets(Extension(mc): Extension<ModelController>) -> TicketsTemplate {
    println!("->> {:<12} - get_tickets", "HANDLER");
    let tickets = mc.get_tickets().await.unwrap_or_default();
    return TicketsTemplate { tickets };
}

async fn create_ticket(
    Extension(mc): Extension<ModelController>,
    Form(payload): Form<TicketPayload>,
) -> TicketTemplate {
    println!("->> {:<12} - create_tickets", "HANDLER");
    let ticket_fc = TicketCreate::new(payload.title, payload.description);

    let ticket = mc.create_ticket(ticket_fc).await.unwrap();

    TicketTemplate { ticket }
}

#[derive(Debug, Deserialize)]
struct TicketPayload {
    title: String,
    description: String,
}
