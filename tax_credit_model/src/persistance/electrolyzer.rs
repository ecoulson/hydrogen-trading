use std::{collections::HashMap, sync::Mutex};

use crate::schema::electrolyzer::Electrolyzer;

pub trait ElectrolyzerPersistanceClient: Send + Sync {
    fn get_electrolyzer(&self, id: usize) -> Result<Electrolyzer, String>;
    fn create_electrolyzer(&self, electrolyzer: &Electrolyzer) -> Result<Electrolyzer, String>;
    fn list_electrolyzers(&self) -> Result<Vec<Electrolyzer>, String>;
}

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
    fn get_electrolyzer(&self, id: usize) -> Result<Electrolyzer, String> {
        Ok(self
            .electrolyzers_by_id
            .lock()
            .expect("Should obtain mutex")
            .get(&id)
            .unwrap()
            .clone())
    }

    fn create_electrolyzer(&self, electrolyzer: &Electrolyzer) -> Result<Electrolyzer, String> {
        let mut electrolyzer = electrolyzer.clone();
        let mut locked_map = self
            .electrolyzers_by_id
            .lock()
            .expect("Should obtain mutex");
        electrolyzer.id = locked_map.len();
        locked_map.insert(electrolyzer.id, electrolyzer);

        Ok(electrolyzer)
    }

    fn list_electrolyzers(&self) -> Result<Vec<Electrolyzer>, String> {
        Ok(self
            .electrolyzers_by_id
            .lock()
            .expect("Should obtain mutex")
            .values()
            .map(|value| value.clone())
            .collect())
    }
}
