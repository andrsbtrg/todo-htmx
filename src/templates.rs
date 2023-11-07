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
    pub view_type: String,
    pub tickets_todo: Vec<Ticket>,
    pub tickets_doing: Vec<Ticket>,
    pub tickets_done: Vec<Ticket>,
}

#[derive(Template)]
#[template(path = "ticket_card.html")]
pub struct TicketTemplate {
    pub ticket: Ticket,
}

#[derive(Template)]
#[template(path = "tickets_table.html")]
pub struct TicketsTable {
    pub tickets_todo: Vec<Ticket>,
    pub tickets_doing: Vec<Ticket>,
    pub tickets_done: Vec<Ticket>,
}

#[derive(Template)]
#[template(path = "tickets_list.html")]
pub struct TicketsList {
    pub tickets_todo: Vec<Ticket>,
    pub tickets_doing: Vec<Ticket>,
    pub tickets_done: Vec<Ticket>,
}

#[derive(Template)]
#[template(path = "create_ticket.html")]
pub struct TicketCreate {}

#[derive(Template)]
#[template(path = "profile.html")]
pub struct ProfileTemplate {
    pub username: String,
    pub user_id: i32,
}
