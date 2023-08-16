use askama::Template;

#[derive(Template)]
#[template(path = "items.html")]
pub struct ItemsTemplate {
    pub counter: i32,
}
