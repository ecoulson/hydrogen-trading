use std::{collections::HashMap, sync::Mutex};

use crate::{
    logic::simulation::SimulationState,
    schema::errors::{Error, Result},
};

pub trait SimulationClient: Send + Sync {
    fn get_simulation_state(&self, simulation_id: &i32) -> Result<SimulationState>;
    fn get_next_id(&self) -> Result<i32>;
    fn insert_simulation_state(
        &self,
        simulation_result: &SimulationState,
    ) -> Result<SimulationState>;
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

impl SimulationClient for InMemorySimulationClient {
    fn get_simulation_state(&self, simulation_id: &i32) -> Result<SimulationState> {
        Ok(self
            .simulation_store
            .lock()
            .map_err(|err| Error::create_poison_error(&err.to_string()))?
            .get(simulation_id)
            .ok_or_else(|| Error::create_not_found_error("No simulation found"))?
            .clone())
    }

    fn get_next_id(&self) -> Result<i32> {
        let mut id = self
            .id
            .lock()
            .map_err(|err| Error::create_poison_error(&err.to_string()))?;
        let copy = id.clone();
        *id += 1;

        return Ok(copy);
    }

    fn insert_simulation_state(
        &self,
        simulation_state: &SimulationState,
    ) -> Result<SimulationState> {
        self.simulation_store
            .lock()
            .map_err(|err| Error::create_poison_error(&err.to_string()))?
            .insert(simulation_state.id, simulation_state.clone());

        Ok(simulation_state.clone())
    }
}
