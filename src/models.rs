use crate::{ctx::Context, Error, Result};

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Sqlite, SqlitePool};

/// The ticket that will be stored and sent to the client
#[derive(Serialize, Clone, Debug, FromRow)]
pub struct Ticket {
    pub id: u32,
    pub creator_id: u32,
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

    pub async fn create_ticket(&self, ctx: Context, ticket_fc: TicketCreate) -> Result<Ticket> {
        let creator_id = ctx.user_id();
        match sqlx::query_as::<Sqlite, Ticket>(
            r#"INSERT INTO tickets (title, status, description, creator_id) VALUES (?,?,?,?) RETURNING *;"#,
        )
        .bind(ticket_fc.title)
        .bind("Open")
        .bind(ticket_fc.description)
        .bind(creator_id)
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

    pub async fn get_tickets(&self, ctx: Context) -> Result<Vec<Ticket>> {
        let _creator_id = ctx.user_id();
        match sqlx::query_as::<_, Ticket>("SELECT * FROM tickets")
            .fetch_all(&self.db)
            .await
        {
            Ok(tickets_result) => Ok(tickets_result),
            Err(_) => Err(Error::NoTicketsFound),
        }
    }

    pub async fn delete_ticket(&self, ctx: Context, id: i32) -> Result<()> {
        let _creator_id = ctx.user_id();
        match sqlx::query!("DELETE from tickets WHERE id=(?)", id)
            .execute(&self.db)
            .await
        {
            Ok(res) => {
                println!("{:?}", res);
                return Ok(());
            }
            Err(e) => {
                eprint!("ERROR: {:?}", e);
                return Err(Error::TicketNotFound);
            }
        }
    }
}
