use askama::Template;

use crate::models::Ticket;

#[derive(Template)]
#[template(path = "start.html")]
pub struct StartTemplate<'a> {
    pub username: &'a str,
}

#[derive(Template)]
#[template(path = "tickets.html")]
pub struct TicketsTemplate {
    pub username: String,
    pub tickets: Vec<Ticket>,
}

#[derive(Template)]
#[template(path = "ticket.html")]
pub struct TicketTemplate {
    pub ticket: Ticket,
}

#[derive(Template)]
#[template(path = "profile.html")]
pub struct ProfileTemplate {}
