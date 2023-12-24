use std::collections::HashMap;

use crate::{
    concurrency::mutex::Mutex,
    schema::{
        errors::{Error, Result},
        simulation::SimulationId,
        user::UserId,
    },
};

pub type SimulationSelection = usize;

pub trait SimulationSelectionClient: Sync + Send {
    fn select(&self, user_id: UserId, simulation_id: SimulationId) -> Result<SimulationSelection>;
    fn current_selection(&self, user_id: &UserId) -> Result<Option<SimulationSelection>>;
    fn expect_current_selection(&self, user_id: &UserId) -> Result<SimulationSelection>;
    fn unselect(&self, user_id: &UserId) -> Result<()>;
}

pub struct InMemorySimulationSelectionClient {
    selection_by_user_id: Mutex<HashMap<UserId, SimulationSelection>>,
}

impl InMemorySimulationSelectionClient {
    pub fn new() -> Self {
        Self {
            selection_by_user_id: Mutex::new(HashMap::new()),
        }
    }
}

impl SimulationSelectionClient for InMemorySimulationSelectionClient {
    fn select(&self, user_id: UserId, simulation_id: SimulationId) -> Result<SimulationSelection> {
        self.selection_by_user_id
            .lock()?
            .insert(user_id, simulation_id);

        Ok(simulation_id)
    }

    fn expect_current_selection(&self, user_id: &UserId) -> Result<SimulationSelection> {
        Ok(self
            .selection_by_user_id
            .lock()?
            .get(user_id)
            .map(|id| id.clone())
            .ok_or_else(|| Error::not_found("No simulation selected for current user"))?)
    }

    fn current_selection(&self, user_id: &UserId) -> Result<Option<SimulationSelection>> {
        Ok(self
            .selection_by_user_id
            .lock()?
            .get(user_id)
            .map(|id| id.clone()))
    }

    fn unselect(&self, user_id: &UserId) -> Result<()> {
        self.selection_by_user_id.lock()?.remove(user_id);

        Ok(())
    }
}
