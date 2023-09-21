use axum::{
    extract::Path,
    routing::{delete, get, post, put},
    Extension, Form, Router,
};
use serde::Deserialize;

use crate::{
    ctx::Context,
    models::{ModelController, Ticket, TicketFC},
    templates::{TicketCreate, TicketTemplate, TicketsTable, TicketsTemplate},
};

pub fn routes() -> Router {
    Router::new()
        .route("/tickets", get(get_tickets))
        .route("/tickets/new", get(ticket_creator))
        .route("/tickets", post(create_ticket))
        .route("/tickets/:id", delete(delete_ticket))
        .route("/tickets/:id/doing", put(update_ticket_doing))
        .route("/tickets/:id/done", put(update_ticket_done))
        .route("/tickets/:id/todo", put(update_ticket_todo))
}

async fn get_tickets(Extension(mc): Extension<ModelController>, ctx: Context) -> TicketsTemplate {
    println!("->> {:<12} - get_tickets", "HANDLER");
    let tickets = mc.get_tickets(&ctx).await.unwrap_or_default();
    let username = ctx.username();

    let mut tickets_todo: Vec<Ticket> = Vec::new();
    let mut tickets_doing: Vec<Ticket> = Vec::new();
    let mut tickets_done: Vec<Ticket> = Vec::new();

    for ticket in tickets {
        match ticket.status.as_str() {
            "to-do" => tickets_todo.push(ticket.to_owned()),
            "doing" => tickets_doing.push(ticket.to_owned()),
            "done" => tickets_done.push(ticket.to_owned()),
            _ => tickets_todo.push(ticket.to_owned()),
        }
    }

    TicketsTemplate {
        username: username.to_string(),
        tickets_todo,
        tickets_doing,
        tickets_done,
    }
}

async fn ticket_creator(_ctx: Context) -> TicketCreate {
    TicketCreate {}
}

async fn create_ticket(
    Extension(mc): Extension<ModelController>,
    ctx: Context,
    Form(payload): Form<TicketPayload>,
) -> TicketTemplate {
    println!("->> {:<12} - create_tickets", "HANDLER");
    if payload.title.is_empty() {}
    let ticket_fc = TicketFC::new(payload.title, payload.description);

    let ticket = mc.create_ticket(ctx, ticket_fc).await.unwrap();

    TicketTemplate { ticket }
}

async fn delete_ticket(
    Extension(mc): Extension<ModelController>,
    ctx: Context,
    Path(id): Path<i32>,
) {
    println!("->> {:<12} - delete_ticket", "HANDLER");
    mc.delete_ticket(ctx, id).await.unwrap();
}

async fn update_ticket_doing(
    Extension(mc): Extension<ModelController>,
    ctx: Context,
    Path(id): Path<i32>,
) -> TicketsTable {
    println!("->> {:<12} - update_ticket", "HANDLER");
    let tickets = mc.update_ticket(&ctx, id, "doing").await.unwrap();

    render_ticket_table(tickets)
}
async fn update_ticket_done(
    Extension(mc): Extension<ModelController>,
    ctx: Context,
    Path(id): Path<i32>,
) -> TicketsTable {
    println!("->> {:<12} - update_ticket", "HANDLER");
    let tickets = mc.update_ticket(&ctx, id, "done").await.unwrap();

    render_ticket_table(tickets)
}

async fn update_ticket_todo(
    Extension(mc): Extension<ModelController>,
    ctx: Context,
    Path(id): Path<i32>,
) -> TicketsTable {
    println!("->> {:<12} - update_ticket", "HANDLER");
    let tickets = mc.update_ticket(&ctx, id, "to-do").await.unwrap();

    render_ticket_table(tickets)
}

fn render_ticket_table(tickets: Vec<Ticket>) -> TicketsTable {
    let mut tickets_todo: Vec<Ticket> = Vec::new();
    let mut tickets_doing: Vec<Ticket> = Vec::new();
    let mut tickets_done: Vec<Ticket> = Vec::new();

    for ticket in tickets {
        match ticket.status.as_str() {
            "to-do" => tickets_todo.push(ticket.to_owned()),
            "doing" => tickets_doing.push(ticket.to_owned()),
            "done" => tickets_done.push(ticket.to_owned()),
            _ => tickets_todo.push(ticket.to_owned()),
        }
    }

    TicketsTable {
        tickets_todo,
        tickets_doing,
        tickets_done,
    }
}

#[derive(Debug, Deserialize)]
/// The payload coming from a Form to create a new ticket
struct TicketPayload {
    title: String,
    description: String,
}
