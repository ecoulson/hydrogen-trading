use std::{collections::HashMap, sync::Mutex};

use crate::schema::{
    errors::{Error, Result},
    simulation_schema::{GenerationMetric, PowerGrid, PowerPlant},
};

pub trait GridClient: Send + Sync {
    fn get_power_grid(&self) -> Result<PowerGrid>;
    fn add_generations(&self, generations: Vec<GenerationMetric>) -> Result<()>;
}

pub struct InMemoryGridFetcher {
    generations_store: Mutex<HashMap<i32, Vec<GenerationMetric>>>,
}

impl InMemoryGridFetcher {
    pub fn new() -> InMemoryGridFetcher {
        InMemoryGridFetcher {
            generations_store: Mutex::new(HashMap::new()),
        }
    }
}

impl GridClient for InMemoryGridFetcher {
    fn get_power_grid(&self) -> Result<PowerGrid> {
        let plant_id = 0;
        let generations = self
            .generations_store
            .lock()
            .map_err(|err| Error::create_poison_error(&err.to_string()))?
            .get(&plant_id)
            .ok_or_else(|| Error::create_not_found_error("No generations found"))?
            .clone();
        let power_plant = PowerPlant {
            plant_id,
            generations,
        };

        Ok(PowerGrid {
            power_plants: vec![power_plant],
        })
    }

    fn add_generations(&self, generations: Vec<GenerationMetric>) -> Result<()> {
        let mut store = self
            .generations_store
            .lock()
            .map_err(|err| Error::create_poison_error(&err.to_string()))?;

        for generation in generations {
            if let Some(exisiting) = store.get_mut(&generation.plant_id) {
                exisiting.push(generation);
            } else {
                store.insert(generation.plant_id, vec![generation]);
            }
        }

        Ok(())
    }
}
