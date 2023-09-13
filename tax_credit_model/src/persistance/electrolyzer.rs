use std::{collections::HashMap, sync::Mutex};

use crate::schema::{electrolyzer::Electrolyzer, errors::Error};

pub trait ElectrolyzerPersistanceClient: Send + Sync {
    fn get_electrolyzer(&self, id: usize) -> Result<Electrolyzer>;
    fn create_electrolyzer(&self, electrolyzer: &Electrolyzer) -> Result<Electrolyzer>;
    fn list_electrolyzers(&self) -> Result<Vec<Electrolyzer>>;
}

type Result<T> = std::result::Result<T, Error>;

pub struct InMemoryElectrolyzerPersistanceClient {
    electrolyzers_by_id: Mutex<HashMap<usize, Electrolyzer>>,
}

impl InMemoryElectrolyzerPersistanceClient {
    pub fn new() -> InMemoryElectrolyzerPersistanceClient {
        InMemoryElectrolyzerPersistanceClient {
            electrolyzers_by_id: Mutex::new(HashMap::new()),
        }
    }
}

impl ElectrolyzerPersistanceClient for InMemoryElectrolyzerPersistanceClient {
    fn get_electrolyzer(&self, id: usize) -> Result<Electrolyzer> {
        Ok(self
            .electrolyzers_by_id
            .lock()
            .map_err(|err| Error::create_poison_error(&err.to_string()))?
            .get(&id)
            .ok_or_else(|| Error::create_not_found_error(&id.to_string()))?
            .clone())
    }

    fn create_electrolyzer(&self, electrolyzer: &Electrolyzer) -> Result<Electrolyzer> {
        let mut electrolyzer = electrolyzer.clone();
        let mut locked_map = self
            .electrolyzers_by_id
            .lock()
            .map_err(|err| Error::create_poison_error(&err.to_string()))?;
        electrolyzer.id = locked_map.len();
        locked_map.insert(electrolyzer.id, electrolyzer);

        Ok(electrolyzer)
    }

    fn list_electrolyzers(&self) -> Result<Vec<Electrolyzer>> {
        Ok(self
            .electrolyzers_by_id
            .lock()
            .map_err(|err| Error::create_poison_error(&err.to_string()))?
            .values()
            .map(|value| value.clone())
            .collect())
    }
}
