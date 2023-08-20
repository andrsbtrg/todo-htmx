use askama::Template;

use crate::models::Ticket;

#[derive(Template)]
#[template(path = "start.html")]
pub struct StartTemplate {
    pub username: String,
}

#[derive(Template)]
#[template(path = "tickets.html")]
pub struct TicketsTemplate {
    pub tickets: Vec<Ticket>,
}

#[derive(Template)]
#[template(path = "ticket.html")]
pub struct TicketTemplate {
    pub ticket: Ticket,
}
