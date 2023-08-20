use axum::{
    routing::{get, post},
    Extension, Router,
};

use crate::{
    models::{ModelController, TicketCreate},
    templates::TicketsTemplate,
};

pub fn routes() -> Router {
    Router::new()
        .route("/tickets", get(get_tickets))
        .route("/tickets", post(create_ticket))
}

async fn get_tickets(Extension(mc): Extension<ModelController>) -> TicketsTemplate {
    println!("->> {:<12} - get_tickets", "HANDLER");
    if let Ok(tickets) = mc.get_tickets().await {
        println!("{:?}", tickets);
    } else {
        println!("No tickets");
    }

    TicketsTemplate {}
}

async fn create_ticket(Extension(mc): Extension<ModelController>) {
    println!("->> {:<12} - create_tickets", "HANDLER");
    let ticket_fc = TicketCreate::new("My first ticket");

    if let Ok(ticket) = mc.create_ticket(ticket_fc).await {
        println!("{:?}", ticket);
    } else {
        println!("No tickets");
    }
}
