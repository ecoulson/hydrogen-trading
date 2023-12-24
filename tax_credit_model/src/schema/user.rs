pub type UserId = usize;

#[derive(Debug, Clone, Default)]
pub struct User {
    pub id: UserId,
}

impl User {
    pub fn new(id: UserId) -> Self {
        Self { id }
    }
}
