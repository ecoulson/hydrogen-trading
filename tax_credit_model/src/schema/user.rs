pub type UserId = usize;

#[derive(Debug, Clone, Default)]
pub struct User {
    id: UserId,
}

impl User {
    pub fn new(id: UserId) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &UserId {
        &self.id
    }

    pub fn set_id(&mut self, user_id: &UserId) {
        self.id = *user_id;
    }
}
