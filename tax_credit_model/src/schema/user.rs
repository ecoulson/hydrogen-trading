pub type UserId = usize;

#[derive(Debug, Clone, Default)]
pub struct User {
    id: UserId,
    simulation_id: i32,
}

impl User {
    pub fn id(&self) -> &UserId {
        &self.id
    }

    pub fn set_id(&mut self, user_id: &UserId) {
        self.id = *user_id;
    }

    pub fn simulation_id(&self) -> i32 {
        self.simulation_id
    }

    pub fn set_simulation_id(&mut self, simulation_id: i32) {
        self.simulation_id = simulation_id;
    }
}
