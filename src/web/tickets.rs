use axum::{
    extract::Path,
    routing::{delete, get, post},
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
        .route("/tickets/:id", delete(delete_ticket))
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

async fn delete_ticket(Extension(mc): Extension<ModelController>, Path(id): Path<i32>) {
    println!("->> {:<12} - delete_ticket", "HANDLER");
    mc.delete_ticket(id).await;
}

#[derive(Debug, Deserialize)]
struct TicketPayload {
    title: String,
    description: String,
}
