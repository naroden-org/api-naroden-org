#[derive(Debug, Clone)]
pub struct NarodenContext {
    user_id: String,
    role: String,
    device_id: String,
    device_type: String,
}

impl NarodenContext {
    pub fn new(user_id: String, role: String, device_id: String, device_type: String) -> Self {
        Self { user_id, role, device_id, device_type }
    }

    pub fn user_id(&self) -> &str {
        &self.user_id
    }

    pub fn role(&self) -> &str {
        &self.role
    }

    pub fn device_id(&self) -> &str {
        &self.device_id
    }

    pub fn device_type(&self) -> &str {
        &self.device_type
    }
}