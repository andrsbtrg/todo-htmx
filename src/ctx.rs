#[derive(Clone, Debug)]
pub struct Context {
    user_id: i32,
    username: String,
}

impl Context {
    pub fn new(user_id: i32, username: &str) -> Self {
        Context {
            user_id,
            username: username.to_string(),
        }
    }
    pub fn user_id(&self) -> i32 {
        self.user_id
    }
    pub fn username(&self) -> &str {
        &self.username
    }
}
