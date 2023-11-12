use std::collections::HashMap;

use crate::{
    concurrency::mutex::Mutex,
    schema::{
        errors::{Error, Result},
        simulation_schema::{GenerationMetric, PowerGrid, PowerPlant},
    },
};

pub trait GridClient: Send + Sync {
    fn get_power_grid(&self) -> Result<PowerGrid>;
    fn add_generations(&self, generations: Vec<GenerationMetric>) -> Result<()>;
}

pub struct InMemoryGridClient {
    generations_store: Mutex<HashMap<i32, Vec<GenerationMetric>>>,
}

impl InMemoryGridClient {
    pub fn new() -> InMemoryGridClient {
        InMemoryGridClient {
            generations_store: Mutex::new(HashMap::new()),
        }
    }
}

impl GridClient for InMemoryGridClient {
    fn get_power_grid(&self) -> Result<PowerGrid> {
        let plant_id = 0;
        let generations = Mutex::lock(&self.generations_store)?
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
        let mut store = Mutex::lock(&self.generations_store)?;

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
