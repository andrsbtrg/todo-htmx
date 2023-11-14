use crate::{ctx::Context, Error, Result};

use chrono::Utc;
use serde::Deserialize;
use sqlx::{PgPool, Postgres};
use std::sync::Mutex;

/// Represents the Ticket that is visualized and sent to the client
#[derive(Clone, Debug, sqlx::FromRow)]
pub struct Ticket {
    pub id: i32,
    pub creator_id: i32,
    pub creator_name: String,
    pub title: String,
    pub status: String,
    pub description: String,
    pub created_at: chrono::DateTime<Utc>,
}

/// Represents a Ticket Stored in 'tickets' table
#[derive(Clone, Debug, sqlx::FromRow)]
pub struct TicketC {
    pub id: i32,
    pub creator_id: i32,
    pub title: String,
    pub status: String,
    pub description: String,
    pub created_at: chrono::DateTime<Utc>,
}

/// This helps create tickets, thus it needs Deserialize
#[derive(Deserialize)]
pub struct TicketFC {
    pub title: String,
    pub status: String,
    pub description: String,
}
impl Ticket {
    pub fn from_ticket_c(ticket_c: TicketC, creator_name: &str) -> Self {
        Ticket {
            id: ticket_c.id,
            creator_id: ticket_c.creator_id,
            creator_name: creator_name.to_string(),
            title: ticket_c.title,
            status: ticket_c.status,
            description: ticket_c.description,
            created_at: ticket_c.created_at,
        }
    }
}
impl TicketFC {
    pub fn new(title: impl Into<String>, description: impl Into<String>) -> Self {
        TicketFC {
            title: title.into(),
            description: description.into(),
            status: "to-do".to_string(),
        }
    }
}

pub struct UserCreate {
    pub username: String,
    pub hashed_password: String,
}

/// model controller
#[derive(Clone)]
pub struct ModelController {
    db: std::sync::Arc<Mutex<PgPool>>,
}

impl ModelController {
    pub fn new(db: PgPool) -> Result<Self> {
        Ok(Self {
            db: std::sync::Arc::new(Mutex::new(db)),
        })
    }

    pub async fn create_ticket(&self, ctx: Context, ticket_fc: TicketFC) -> Result<Ticket> {
        let creator_id = ctx.user_id();

        let pool = self.db.lock().unwrap().to_owned();
        match sqlx::query_as::<Postgres, TicketC>(
            r#"INSERT INTO tickets (title, status, description, creator_id, created_at) VALUES ($1,$2,$3,$4,$5) RETURNING *;"#,
        )
        .bind(ticket_fc.title)
        .bind(ticket_fc.status)
        .bind(ticket_fc.description)
        .bind(creator_id)
        .bind(Utc::now())
        .fetch_one(&pool)
        .await
        {
            Ok(ticket) => {
                println!("ticket {} - New ticket has been created", ticket.id);

                Ok( Ticket {
                    creator_name: ctx.username().to_string(),
                    id: ticket.id,
                    creator_id: ticket.creator_id,
                    title: ticket.title,
                    status: ticket.status,
                    description: ticket.description,
                    created_at: ticket.created_at,
                    })
            }
            Err(e) => {
                eprintln!("Failed to execute query: {:?}", e);
                return Err(Error::TicketCreationFailed);
            }
        }
    }

    pub async fn get_tickets(&self, ctx: &Context) -> Result<Vec<Ticket>> {
        let _creator_id = ctx.user_id();

        let pool = self.db.lock().unwrap().to_owned();
        match sqlx::query_as::<Postgres, Ticket>(r#"
SELECT tickets.id, tickets.title, tickets.status, tickets.description, tickets.created_at, tickets.creator_id, users.username AS creator_name
FROM tickets
INNER JOIN users ON tickets.creator_id = users.user_id;"#)
            .fetch_all(&pool)
            .await
        {
            Ok(tickets_result) => Ok(tickets_result),
            Err(_) => Err(Error::NoTicketsFound),
        }
    }

    #[allow(unused)]
    pub async fn get_user_tickets(&self, ctx: Context) -> Result<Vec<Ticket>> {
        let pool = self.db.lock().unwrap().to_owned();
        let _creator_id = ctx.user_id();

        let username = ctx.username();
        match sqlx::query_as::<Postgres, TicketC>("SELECT * FROM tickets WHERE creator_id=($1)")
            .bind(_creator_id)
            .fetch_all(&pool)
            .await
        {
            Ok(tickets_result) => Ok(tickets_result
                .iter()
                .map(|ticket| Ticket::from_ticket_c(ticket.to_owned(), username))
                .collect()),
            Err(_) => Err(Error::NoTicketsFound),
        }
    }

    pub async fn delete_ticket(&self, ctx: Context, id: i32) -> Result<()> {
        let pool = self.db.lock().unwrap().to_owned();
        let _creator_id = ctx.user_id();
        match sqlx::query("DELETE from tickets WHERE id = $1")
            .bind(id)
            .execute(&pool)
            .await
        {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => {
                eprint!("ERROR: {:?}", e);
                return Err(Error::TicketNotFound);
            }
        }
    }

    pub async fn register_new(&self, payload: UserCreate) -> Result<i32> {
        let pool = self.db.lock().unwrap().to_owned();
        let user_id: i32 = sqlx::query_scalar(
            r#"INSERT INTO users (username, password) VALUES ($1, $2) RETURNING user_id ;"#,
        )
        .bind(&payload.username)
        .bind(&payload.hashed_password)
        .fetch_one(&pool)
        .await
        .map_err(|err| {
            eprintln!("Error creating user: {:?}", err);
            Error::UserCreateFail
        })?;

        Ok(user_id)
    }

    pub async fn get_pwd(&self, username: &str) -> Option<String> {
        let pool = self.db.lock().unwrap().to_owned();
        sqlx::query_scalar(r#"SELECT password FROM users WHERE username=($1);"#)
            .bind(&username)
            .fetch_optional(&pool)
            .await
            .unwrap()
    }

    pub async fn get_user_id(&self, username: &str) -> Result<i32> {
        let pool = self.db.lock().unwrap().to_owned();
        let user_id: i32 = sqlx::query_scalar(r#"SELECT user_id FROM users WHERE username=($1);"#)
            .bind(username)
            .fetch_one(&pool)
            .await
            .map_err(|_| Error::UserCreateFail)?;

        Ok(user_id)
    }

    pub async fn get_username(&self, user_id: i32) -> Result<String> {
        let pool = self.db.lock().unwrap().to_owned();
        let s: String = sqlx::query_scalar(r#"SELECT username FROM users where user_id=($1);"#)
            .bind(user_id)
            .fetch_one(&pool)
            .await
            .map_err(|_| Error::UserIdNotFound)?;
        Ok(s)
    }

    pub async fn username_exists(&self, username: &str) -> bool {
        let pool = self.db.lock().unwrap().to_owned();
        match sqlx::query_scalar(r#"SELECT EXISTS(SELECT 1 FROM users WHERE username=($1));"#)
            .bind(username)
            .fetch_one(&pool)
            .await
        {
            Ok(exists) => exists,
            Err(_) => false,
        }
    }

    pub async fn update_ticket(&self, _ctx: &Context, id: i32, arg: &str) -> Result<()> {
        let pool = self.db.lock().unwrap().to_owned();

        let transaction = pool
            .begin()
            .await
            .map_err(|_| Error::DatabaseError)
            .unwrap();
        // Perform the UPDATE operation
        match sqlx::query("UPDATE tickets SET status = ($1) WHERE id = ($2);")
            .bind(arg)
            .bind(id)
            .execute(&pool)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => {
                // Rollback the transaction on UPDATE error
                let _ = transaction
                    .rollback()
                    .await
                    .map_err(|_| Error::DatabaseError);
                eprintln!("{:?}", e);
                Err(Error::UpdateTicketError)
            }
        }
    }

    pub async fn get_ticket(&self, ctx: &Context, id: i32) -> Result<Ticket> {
        let _creator_id = ctx.user_id();

        let pool = self.db.lock().unwrap().to_owned();

        match sqlx::query_as::<Postgres, Ticket>(r#"
SELECT tickets.id, tickets.title, tickets.status, tickets.description, tickets.created_at, tickets.creator_id, users.username AS creator_name
FROM tickets
INNER JOIN users ON tickets.creator_id = users.user_id
WHERE tickets.id = $1;
        "#)
            .bind(id)
            .fetch_one(&pool)
            .await
        {
            Ok(ticket) => Ok(ticket),
            Err(_) => Err(Error::NoTicketsFound),
        }
    }

    pub async fn update_ticket_description(
        &self,
        _ctx: &Context,
        id: i32,
        description: String,
    ) -> Result<()> {
        let pool = self.db.lock().unwrap().to_owned();
        dbg!(id);

        let transaction = pool
            .begin()
            .await
            .map_err(|_| Error::DatabaseError)
            .unwrap();
        // Perform the UPDATE operation
        match sqlx::query("UPDATE tickets SET description = ($1) WHERE id = ($2)")
            .bind(description)
            .bind(id)
            .execute(&pool)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => {
                // Rollback the transaction on UPDATE error
                let _ = transaction
                    .rollback()
                    .await
                    .map_err(|_| Error::DatabaseError);
                eprintln!("{:?}", e);
                Err(Error::UpdateTicketError)
            }
        }
    }
}
