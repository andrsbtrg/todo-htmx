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
    templates::{
        EditTicketDescription, TicketCreate, TicketTemplate, TicketsList, TicketsTable,
        TicketsTemplate,
    },
};

pub fn routes() -> Router {
    Router::new()
        .route("/tickets", get(get_tickets))
        .route("/tickets/:id", get(get_ticket_by_id))
        .route("/tickets/new", get(ticket_creator))
        .route("/tickets/:id/edit_description", get(edit_description))
        .route("/tickets", post(create_ticket))
        .route("/tickets/:id", delete(delete_ticket))
        .route("/tickets/:id", put(update_ticket))
        .route("/tickets/:id/description", post(add_ticket_data))
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

#[derive(Deserialize)]
enum Status {
    #[serde(rename = "doing")]
    Doing,
    #[serde(rename = "to-do")]
    Todo,
    #[serde(rename = "done")]
    Done,
}
impl Status {
    fn as_str(&self) -> &str {
        match self {
            Status::Doing => "doing",
            Status::Todo => "to-do",
            Status::Done => "done",
        }
    }
}

#[derive(Deserialize)]
struct DescriptionUpdate {
    pub description: String,
}

#[derive(Deserialize)]
struct StatusUpdate {
    status: Status,
}

#[derive(Debug, Deserialize, Clone)]
enum CreatedBy {
    #[serde(rename = "everyone")]
    Everyone,
    #[serde(rename = "me")]
    Me,
}

#[derive(Debug, Deserialize)]
pub struct ViewParams {
    view: Option<View>,
    created_by: Option<CreatedBy>,
}
async fn get_tickets(
    Extension(mc): Extension<ModelController>,
    ctx: Context,
    Query(view_params): Query<ViewParams>,
    headers: HeaderMap,
) -> impl IntoResponse {
    println!("->> {:<12} - get_tickets", "HANDLER");

    let created_by = view_params.created_by.unwrap_or(CreatedBy::Everyone);

    let mut tickets = match created_by {
        CreatedBy::Everyone => mc.get_tickets(&ctx).await.unwrap(),
        CreatedBy::Me => mc.get_user_tickets(&ctx).await.unwrap(),
    };

    tickets.sort_by(|a, b| a.created_at.partial_cmp(&b.created_at).unwrap());

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

    match view_params.view {
        Some(view) => match headers.get("hx-request") {
            Some(_) => match view {
                View::Table => TicketsTable {
                    tickets_todo,
                    tickets_doing,
                    tickets_done,
                }
                .into_response(),
                View::List => TicketsList {
                    tickets_todo,
                    tickets_doing,
                    tickets_done,
                }
                .into_response(),
            },
            None => TicketsTemplate {
                username: username.to_string(),
                view_type: view,
                tickets_todo,
                tickets_doing,
                tickets_done,
            }
            .into_response(),
        },
        None => {
            return TicketsTemplate {
                username: username.to_string(),
                view_type: View::Table,
                tickets_todo,
                tickets_doing,
                tickets_done,
            }
            .into_response();
        }
    }
}

async fn get_ticket_by_id(
    Extension(mc): Extension<ModelController>,
    ctx: Context,
    Path(id): Path<i32>,
) -> TicketTemplate {
    println!("->> {:<12} - get_tickets", "HANDLER");

    let ticket = mc.get_ticket(&ctx, id).await.unwrap();
    TicketTemplate { ticket }
}

async fn ticket_creator(_ctx: Context) -> TicketCreate {
    TicketCreate {}
}

async fn edit_description(_ctx: Context, Path(id): Path<i32>) -> EditTicketDescription {
    EditTicketDescription { ticket_id: id }
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

async fn update_ticket(
    Extension(mc): Extension<ModelController>,
    ctx: Context,
    Path(id): Path<i32>,
    headers: HeaderMap,
    axum::extract::Form(payload): axum::extract::Form<StatusUpdate>,
) -> impl IntoResponse {
    println!("->> {:<12} - update_ticket", "HANDLER");

    let referer: String = headers.get(REFERER).unwrap().to_str().unwrap().to_string();

    let _ = mc
        .update_ticket(&ctx, id, payload.status.as_str())
        .await
        .unwrap();
    let tickets = mc.get_tickets(&ctx).await.unwrap();

    render_ticket_table(tickets, &referer)
}
async fn add_ticket_data(
    Extension(mc): Extension<ModelController>,
    ctx: Context,
    Path(id): Path<i32>,
    Form(payload): Form<DescriptionUpdate>,
) -> TicketTemplate {
    println!("->> {:<12} - update_ticket", "HANDLER");

    let status = payload.description;

    let _ = mc
        .update_ticket_description(&ctx, id, status)
        .await
        .unwrap();

    let ticket = mc.get_ticket(&ctx, id).await.unwrap();
    TicketTemplate { ticket }
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
