#[derive(Clone, Debug)]
pub struct Context {
    user_id: u32,
}

impl Context {
    pub fn new(user_id: u32) -> Self {
        Context { user_id }
    }
    pub fn user_id(&self) -> u32 {
        self.user_id
    }
}
