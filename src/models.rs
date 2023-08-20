use crate::{Error, Result};

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Sqlite, SqlitePool};

/// The ticket that will be stored and sent to the client
#[derive(Serialize, Clone, Debug, FromRow)]
pub struct Ticket {
    pub id: u32,
    pub title: String,
    pub status: String,
    pub description: String,
}

/// This helps create tickets, thus it needs Deserialize
#[derive(Deserialize)]
pub struct TicketCreate {
    pub title: String,
    pub status: String,
    pub description: String,
}

impl TicketCreate {
    pub fn new(title: impl Into<String>, description: impl Into<String>) -> Self {
        TicketCreate {
            title: title.into(),
            description: description.into(),
            status: "Open".to_string(),
        }
    }
}

/// model controller
#[derive(Clone)]
pub struct ModelController {
    db: SqlitePool,
}

impl ModelController {
    pub fn new(db: SqlitePool) -> Result<Self> {
        Ok(Self { db })
    }

    pub async fn create_ticket(&self, ticket_fc: TicketCreate) -> Result<Ticket> {
        match sqlx::query_as::<Sqlite, Ticket>(
            r#"INSERT INTO tickets (title, status, description) VALUES (?,?,?) RETURNING *"#,
        )
        .bind(ticket_fc.title)
        .bind("Open")
        .bind(ticket_fc.description)
        .fetch_one(&self.db)
        .await
        {
            Ok(ticket) => {
                println!("ticket {} - New ticket has been created", ticket.id);
                return Ok(ticket);
            }
            Err(e) => {
                println!("Failed to execute query: {:?}", e);
                return Err(Error::TicketCreationFailed);
            }
        }
    }

    pub async fn get_tickets(&self) -> Result<Vec<Ticket>> {
        match sqlx::query_as::<_, Ticket>("SELECT * FROM tickets")
            .fetch_all(&self.db)
            .await
        {
            Ok(tickets_result) => Ok(tickets_result),
            Err(_) => Err(Error::NoTicketsFound),
        }
    }
}
