use crate::schema::{
    simulation_schema::{EnergySource, GenerationMetric, PowerGrid, PowerPlant},
    time::Timestamp,
};

pub trait GridFetcher: Send + Sync {
    fn get_power_grid(&self) -> PowerGrid;
}

pub struct InMemoryGridFetcher {}

impl GridFetcher for InMemoryGridFetcher {
    fn get_power_grid(&self) -> PowerGrid {
        PowerGrid {
            power_plants: vec![self.create_power_plant()],
        }
    }
}

impl InMemoryGridFetcher {
    pub fn new() -> InMemoryGridFetcher {
        InMemoryGridFetcher {}
    }

    fn create_power_plant(&self) -> PowerPlant {
        let mut power_plant = PowerPlant {
            plant_id: 50098,
            energy_source: EnergySource::NaturalGas,
            heat_rate: 0.2,
            generation: vec![],
        };

        power_plant.add_generation(self.generate_power(
            power_plant.plant_id,
            15706.68,
            1351.545,
            0.02,
            Timestamp::default(),
        ));
        power_plant.add_generation(self.generate_power(
            power_plant.plant_id,
            15706.68,
            1351.545,
            0.02,
            Timestamp::default(),
        ));

        power_plant
    }

    fn generate_power(
        &self,
        plant_id: i32,
        amount_mmbtu: f32,
        power_produced_mwh: f32,
        sale_price_usd_per_mwh: f32,
        timestamp: Timestamp,
    ) -> GenerationMetric {
        GenerationMetric {
            plant_id,
            amount_mmbtu,
            amount_mwh: power_produced_mwh,
            time_generated: timestamp,
            sale_price_usd_per_mwh,
        }
    }
}
