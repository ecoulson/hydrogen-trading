use rocket::async_trait;

use crate::{
    persistance::db::DatabaseClient,
    schema::{
        errors::{Error, Result},
        simulation_schema::{GenerationMetric, PowerGrid, PowerPlant},
    },
};

const SALE_PRICE_COLUMN: &'static str = "sale_price";
const TIME_GENERATED_COLUMN: &'static str = "time_generated";
const PORTFOLIO_COLUMN: &'static str = "portfolio";
const ID_COLUMN: &'static str = "id";
const PLANT_ID_COLUMN: &'static str = "plant_id";

// TODO: Move from a fetcher to a persistance client
#[async_trait]
pub trait GridFetcher: Send + Sync {
    async fn get_power_grid(&self, client: &mut DatabaseClient) -> Result<PowerGrid>;
}

pub struct SQLGridFetcher {}

impl SQLGridFetcher {
    pub fn new() -> SQLGridFetcher {
        SQLGridFetcher {}
    }
}

#[async_trait]
impl GridFetcher for SQLGridFetcher {
    async fn get_power_grid(&self, client: &mut DatabaseClient) -> Result<PowerGrid> {
        let mut power_plant = PowerPlant {
            plant_id: 0,
            generation: vec![],
        };
        let rows = client
            .query(
                "SELECT * FROM GenerationMetrics WHERE plant_id = $1",
                &[&power_plant.plant_id],
            )
            .await?;

        power_plant.generation = rows
            .iter()
            .map(|row| {
                Ok(GenerationMetric {
                    id: row.get(ID_COLUMN),
                    plant_id: row.get(PLANT_ID_COLUMN),
                    sale_price_usd_per_mwh: row.get(SALE_PRICE_COLUMN),
                    time_generated: serde_json::from_value(row.get(TIME_GENERATED_COLUMN))
                        .map_err(|err| Error::create_invalid_argument_error(&err.to_string()))?,
                    portfolio: serde_json::from_value(row.get(PORTFOLIO_COLUMN))
                        .map_err(|err| Error::create_invalid_argument_error(&err.to_string()))?,
                })
            })
            .collect::<Result<_>>()?;

        Ok(PowerGrid {
            power_plants: vec![power_plant],
        })
    }
}
