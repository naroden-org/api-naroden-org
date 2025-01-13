#[derive(Debug, Clone)]
pub struct NarodenContext {
    user_id: String,
    role: String,
}

impl NarodenContext {
    pub fn new(user_id: String, role: String) -> Self {
        Self { user_id, role }
    }

    pub fn user_id(&self) -> &str {
        &self.user_id
    }

    pub fn role(&self) -> &str {
        &self.role
    }
}