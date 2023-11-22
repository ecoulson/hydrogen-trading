use std::collections::HashMap;

use crate::{
    concurrency::mutex::Mutex,
    schema::{
        electrolyzer::Electrolyzer,
        errors::{Error, Result},
    },
};

pub trait ElectrolyzerClient: Send + Sync {
    fn get_electrolyzer(&self, id: usize) -> Result<Electrolyzer>;
    fn create_electrolyzer(&self, electrolyzer: &Electrolyzer) -> Result<Electrolyzer>;
    fn list_electrolyzers(&self) -> Result<Vec<Electrolyzer>>;
}

pub struct InMemoryElectrolyzerPersistanceClient {
    electrolyzers_by_id: Mutex<HashMap<usize, Electrolyzer>>,
}

impl InMemoryElectrolyzerPersistanceClient {
    pub fn new() -> Self {
        Self {
            electrolyzers_by_id: Mutex::new(HashMap::new()),
        }
    }
}

impl ElectrolyzerClient for InMemoryElectrolyzerPersistanceClient {
    fn get_electrolyzer(&self, id: usize) -> Result<Electrolyzer> {
        Ok(Mutex::lock(&self.electrolyzers_by_id)?
            .get(&id)
            .ok_or_else(|| Error::not_found(&id.to_string()))?
            .clone())
    }

    fn create_electrolyzer(&self, electrolyzer: &Electrolyzer) -> Result<Electrolyzer> {
        let mut electrolyzer = electrolyzer.clone();
        let mut locked_map = Mutex::lock(&self.electrolyzers_by_id)?;
        electrolyzer.id = locked_map.len();
        locked_map.insert(electrolyzer.id, electrolyzer.clone());

        Ok(electrolyzer)
    }

    fn list_electrolyzers(&self) -> Result<Vec<Electrolyzer>> {
        Ok(Mutex::lock(&self.electrolyzers_by_id)?
            .values()
            .map(|value| value.clone())
            .collect())
    }
}
