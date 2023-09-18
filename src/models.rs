use crate::{ctx::Context, Error, Result};

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Sqlite, SqlitePool};
use std::sync::Mutex;

/// Represents the Ticket that is visualized and sent to the client
#[derive(Serialize, Clone, Debug, FromRow)]
pub struct Ticket {
    pub id: u32,
    pub creator_id: u32,
    pub creator_name: String,
    pub title: String,
    pub status: String,
    pub description: String,
}

/// Represents a Ticket Stored in DB
#[derive(Serialize, Clone, Debug, FromRow)]
pub struct TicketC {
    pub id: u32,
    pub creator_id: u32,
    pub title: String,
    pub status: String,
    pub description: String,
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
    db: std::sync::Arc<Mutex<SqlitePool>>,
}

impl ModelController {
    pub fn new(db: SqlitePool) -> Result<Self> {
        Ok(Self {
            db: std::sync::Arc::new(Mutex::new(db)),
        })
    }

    pub async fn create_ticket(&self, ctx: Context, ticket_fc: TicketFC) -> Result<Ticket> {
        let creator_id = ctx.user_id();

        let pool = self.db.lock().unwrap().to_owned();
        match sqlx::query_as::<Sqlite, TicketC>(
            r#"INSERT INTO tickets (title, status, description, creator_id) VALUES (?,?,?,?) RETURNING *;"#,
        )
        .bind(ticket_fc.title)
        .bind(ticket_fc.status)
        .bind(ticket_fc.description)
        .bind(creator_id)
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
                    })
            }
            Err(e) => {
                println!("Failed to execute query: {:?}", e);
                return Err(Error::TicketCreationFailed);
            }
        }
    }

    pub async fn get_tickets(&self, ctx: &Context) -> Result<Vec<Ticket>> {
        let _creator_id = ctx.user_id();

        let pool = self.db.lock().unwrap().to_owned();
        match sqlx::query_as::<_, Ticket>(r#"
SELECT tickets.id, tickets.title, tickets.status, tickets.description, tickets.creator_id, users.username AS creator_name
FROM tickets
INNER JOIN users ON tickets.creator_id = users.user_id;
        "#)
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
        match sqlx::query_as::<_, TicketC>("SELECT * FROM tickets WHERE creator_id=(?)")
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
        match sqlx::query!("DELETE from tickets WHERE id=(?)", id)
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

    pub async fn register_new(&self, payload: UserCreate) -> Result<u32> {
        let pool = self.db.lock().unwrap().to_owned();
        let user_id: u32 = sqlx::query_scalar(
            r#"INSERT INTO users (username, password) VALUES (?, ?) RETURNING user_id ;"#,
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
        sqlx::query_scalar(r#"SELECT password FROM users WHERE username=(?);"#)
            .bind(&username)
            .fetch_optional(&pool)
            .await
            .unwrap()
    }

    pub async fn get_user_id(&self, username: &str) -> Result<u32> {
        let pool = self.db.lock().unwrap().to_owned();
        let user_id: u32 = sqlx::query_scalar(r#"SELECT user_id FROM users WHERE username=(?);"#)
            .bind(username)
            .fetch_one(&pool)
            .await
            .map_err(|_| Error::UserCreateFail)?;

        Ok(user_id)
    }

    pub async fn get_username(&self, user_id: u32) -> Result<String> {
        let pool = self.db.lock().unwrap().to_owned();
        let s: String = sqlx::query_scalar(r#"SELECT username FROM users where user_id=(?);"#)
            .bind(user_id)
            .fetch_one(&pool)
            .await
            .map_err(|_| Error::UserIdNotFound)?;
        Ok(s)
    }

    pub async fn username_exists(&self, username: &str) -> bool {
        let pool = self.db.lock().unwrap().to_owned();
        match sqlx::query_scalar(r#"SELECT EXISTS(SELECT 1 FROM users WHERE username=(?));"#)
            .bind(username)
            .fetch_one(&pool)
            .await
        {
            Ok(exists) => exists,
            Err(_) => false,
        }
    }

    pub async fn update_ticket(&self, _ctx: &Context, id: i32, arg: &str) -> Result<Vec<Ticket>> {
        let pool = self.db.lock().unwrap().to_owned();
        // We need to reuse the same pool, not clone it so both trx can happen
        let transaction = pool
            .begin()
            .await
            .map_err(|_| Error::DatabaseError)
            .unwrap();

        match sqlx::query_as::<_, Ticket>(
            r#"
UPDATE tickets SET status=(?) WHERE id=(?);
SELECT tickets.id, tickets.title, tickets.status, tickets.description, tickets.creator_id, users.username AS creator_name
FROM tickets
INNER JOIN users ON tickets.creator_id = users.user_id;
            "#,
        )
        .bind(arg)
        .bind(id)
        .fetch_all(&pool)
        .await
        {
            Ok(_tickets) => Ok(_tickets),
            Err(e) => {
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
