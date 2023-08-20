use askama::Template;

#[derive(Template)]
#[template(path = "start.html")]
pub struct StartTemplate {
    pub username: String,
}

#[derive(Template)]
#[template(path = "tickets.html")]
pub struct TicketsTemplate {}
