use askama_axum::IntoResponse;
use axum::{
    extract::{Path, Query},
    http::{header::REFERER, HeaderMap},
    routing::{delete, get, post, put},
    Extension, Form, Router,
};
use serde::Deserialize;

use crate::{
    ctx::Context,
    models::{ModelController, Ticket, TicketFC},
    templates::{TicketCreate, TicketTemplate, TicketsList, TicketsTable, TicketsTemplate},
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

/// View options
/// TODO: try https://github.com/tokio-rs/axum/blob/main/examples/query-params-with-empty-strings/src/main.rs
#[derive(Deserialize, Debug)]
pub enum View {
    #[serde(rename = "table")]
    Table,
    #[serde(rename = "list")]
    List,
}

#[derive(Debug, Deserialize)]
pub struct ViewParams {
    view: Option<View>,
}
async fn get_tickets(
    Extension(mc): Extension<ModelController>,
    ctx: Context,
    Query(view_params): Query<ViewParams>,
) -> TicketsTemplate {
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

    let view: View = view_params.view.unwrap_or(View::Table);

    TicketsTemplate {
        username: username.to_string(),
        view_type: view,
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

    dbg!(&ticket);

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
    headers: HeaderMap,
) -> impl IntoResponse {
    println!("->> {:<12} - update_ticket", "HANDLER");

    let referer: String = headers.get(REFERER).unwrap().to_str().unwrap().to_string();

    let tickets = mc.update_ticket(&ctx, id, "doing").await.unwrap();

    println!("{}", referer);

    render_ticket_table(tickets, &referer)
}
async fn update_ticket_done(
    Extension(mc): Extension<ModelController>,
    ctx: Context,
    Path(id): Path<i32>,
    headers: HeaderMap,
) -> impl IntoResponse {
    println!("->> {:<12} - update_ticket", "HANDLER");
    let tickets = mc.update_ticket(&ctx, id, "done").await.unwrap();

    let referer: String = headers.get(REFERER).unwrap().to_str().unwrap().to_string();

    println!("{}", referer);
    render_ticket_table(tickets, &referer)
}

async fn update_ticket_todo(
    Extension(mc): Extension<ModelController>,
    ctx: Context,
    Path(id): Path<i32>,
    headers: HeaderMap,
) -> impl IntoResponse {
    println!("->> {:<12} - update_ticket", "HANDLER");
    let tickets = mc.update_ticket(&ctx, id, "to-do").await.unwrap();

    let referer: String = headers.get(REFERER).unwrap().to_str().unwrap().to_string();

    println!("{}", referer);
    render_ticket_table(tickets, &referer)
}

fn render_ticket_table(tickets: Vec<Ticket>, referer: &str) -> impl IntoResponse {
    let mut tickets_todo: Vec<Ticket> = Vec::with_capacity(tickets.len());
    let mut tickets_doing: Vec<Ticket> = Vec::with_capacity(tickets.len());
    let mut tickets_done: Vec<Ticket> = Vec::with_capacity(tickets.len());

    for ticket in tickets.into_iter() {
        match ticket.status.as_str() {
            "to-do" => tickets_todo.push(ticket),
            "doing" => tickets_doing.push(ticket),
            "done" => tickets_done.push(ticket),
            _ => tickets_todo.push(ticket),
        }
    }

    match referer.contains(r#"list"#) {
        true => {
            return TicketsList {
                tickets_todo,
                tickets_doing,
                tickets_done,
            }
            .into_response()
        }
        false => TicketsTable {
            tickets_todo,
            tickets_doing,
            tickets_done,
        }
        .into_response(),
    }
}

#[derive(Debug, Deserialize)]
/// The payload coming from a Form to create a new ticket
struct TicketPayload {
    title: String,
    description: String,
}
