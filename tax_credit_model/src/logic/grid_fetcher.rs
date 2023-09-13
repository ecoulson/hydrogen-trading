use std::sync::Mutex;

use crate::schema::{
    errors::{Error, Result},
    simulation_schema::{EnergySourcePortfolio, GenerationMetric, PowerGrid, PowerPlant},
    time::Timestamp,
};

// TODO: Move from a fetcher to a persistance client
pub trait GridFetcher: Send + Sync {
    fn get_power_grid(&self) -> Result<PowerGrid>;
    fn add_generations(&self, plant_id: i32, generation: &mut Vec<GenerationMetric>) -> Result<()>;
}

pub struct InMemoryGridFetcher {
    power_grid: Mutex<PowerGrid>,
}

impl GridFetcher for InMemoryGridFetcher {
    fn get_power_grid(&self) -> Result<PowerGrid> {
        Ok(self
            .power_grid
            .lock()
            .map_err(|err| Error::create_poison_error(&err.to_string()))?
            .clone())
    }

    fn add_generations(
        &self,
        plant_id: i32,
        generations: &mut Vec<GenerationMetric>,
    ) -> Result<()> {
        self.power_grid
            .lock()
            .map_err(|err| Error::create_poison_error(&err.to_string()))?
            .power_plants
            .iter_mut()
            .find(|plant| plant.plant_id == plant_id)
            .ok_or_else(|| Error::create_not_found_error(&format!("plant id {}", plant_id)))?
            .generation
            .append(generations);

        Ok(())
    }
}

impl InMemoryGridFetcher {
    pub fn new() -> InMemoryGridFetcher {
        InMemoryGridFetcher {
            power_grid: Mutex::new(PowerGrid {
                power_plants: vec![InMemoryGridFetcher::create_power_plant()],
            }),
        }
    }

    fn create_power_plant() -> PowerPlant {
        let mut power_plant = PowerPlant {
            plant_id: 50098,
            generation: vec![],
        };
        let mut portfolio = EnergySourcePortfolio::default();
        portfolio.total_electricity_mwh = 10.0;
        portfolio.natural_gas_mwh = 10.0;
        power_plant.generation.push(GenerationMetric {
            plant_id: power_plant.plant_id,
            time_generated: Timestamp::default(),
            sale_price_usd_per_mwh: 0.02,
            portfolio,
        });

        power_plant
    }
}
