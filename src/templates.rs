use askama::Template;

#[derive(Template)]
#[template(path = "items.html")]
pub struct ItemsTemplate {
    pub counter: i32,
}

#[derive(Template)]
#[template(path = "header_user.html")]
pub struct HeaderUserTemplate {
    pub name: String,
}

#[derive(Template)]
#[template(path = "start.html")]
pub struct StartTemplate {}
