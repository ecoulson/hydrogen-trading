use std::collections::HashMap;

use crate::{
    concurrency::mutex::Mutex,
    logic::simulation::SimulationState,
    schema::errors::{Error, Result},
};

pub trait SimulationClient: Send + Sync {
    fn get_simulation_state(&self, simulation_id: &i32) -> Result<SimulationState>;
    fn create_simulation_state(
        &self,
        simulation_state: &SimulationState,
    ) -> Result<SimulationState>;
    fn ensure_simulation_exists(&self, simulation_id: &i32) -> Result<SimulationState>;
    fn list_simulations(&self) -> Result<Vec<SimulationState>>;
    fn update(&self, simulation_state: &SimulationState) -> Result<SimulationState>;
}

pub struct InMemorySimulationClient {
    simulation_store: Mutex<HashMap<i32, SimulationState>>,
    id: Mutex<i32>,
}

impl InMemorySimulationClient {
    pub fn new() -> InMemorySimulationClient {
        InMemorySimulationClient {
            simulation_store: Mutex::new(HashMap::new()),
            id: Mutex::new(0),
        }
    }
}

impl InMemorySimulationClient {
    fn get_next_id(&self) -> Result<i32> {
        let mut id = Mutex::lock(&self.id)?;
        let copy = id.clone();
        *id += 1;

        Ok(copy)
    }
}

impl SimulationClient for InMemorySimulationClient {
    fn get_simulation_state(&self, simulation_id: &i32) -> Result<SimulationState> {
        Ok(Mutex::lock(&self.simulation_store)?
            .get(simulation_id)
            .ok_or_else(|| Error::create_not_found_error("No simulation found"))?
            .clone())
    }

    fn ensure_simulation_exists(&self, simulation_id: &i32) -> Result<SimulationState> {
        let simulations_by_id = Mutex::lock(&self.simulation_store)?;

        if let Some(simulation_state) = simulations_by_id.get(&simulation_id) {
            Ok(simulation_state.clone())
        } else {
            drop(simulations_by_id);
            self.create_simulation_state(&SimulationState::default())
        }
    }

    fn create_simulation_state(
        &self,
        simulation_state: &SimulationState,
    ) -> Result<SimulationState> {
        let mut simulation_state = simulation_state.clone();
        simulation_state.id = self.get_next_id()?;
        Mutex::lock(&self.simulation_store)?.insert(simulation_state.id, simulation_state.clone());

        Ok(simulation_state)
    }

    fn update(&self, simulation_state: &SimulationState) -> Result<SimulationState> {
        Mutex::lock(&self.simulation_store)?.insert(simulation_state.id, simulation_state.clone());

        Ok(simulation_state.clone())
    }

    fn list_simulations(&self) -> Result<Vec<SimulationState>> {
        Ok(Mutex::lock(&self.simulation_store)?
            .iter()
            .map(|(_, state)| state.clone())
            .collect())
    }
}
