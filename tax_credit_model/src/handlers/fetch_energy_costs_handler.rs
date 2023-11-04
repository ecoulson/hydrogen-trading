use std::collections::HashMap;

use rocket::{get, serde::json::Json, State};

use crate::{
    persistance::simulation::SimulationClient,
    schema::time_series::{ChartColor, TimeSeries, TimeSeriesEntry},
};

#[get("/fetch_energy_costs/<simulation_id>")]
pub fn fetch_energy_costs_handler(
    simulation_id: i32,
    simulation_client: &State<Box<dyn SimulationClient>>,
) -> Result<Json<TimeSeries>, String> {
    let simulation = simulation_client
        .get_simulation_state(&simulation_id)
        .map_err(|err| err.to_string())?;
    let mut energy_costs_time_series = TimeSeries {
        label: String::from("Energy Cost"),
        color: ChartColor::Blue,
        data_points: simulation
            .transactions
            .iter()
            .fold(HashMap::new(), |mut aggregation, transaction| {
                if let Some(current_price) = aggregation.get_mut(&transaction.timestamp) {
                    *current_price += transaction.price_usd;
                } else {
                    aggregation.insert(transaction.timestamp, transaction.price_usd);
                }

                aggregation
            })
            .iter()
            .map(|(key, value)| {
                Ok(TimeSeriesEntry {
                    color: ChartColor::Blue,
                    date: key.to_utc_date_time()?.to_rfc3339(),
                    value: *value,
                })
            })
            .collect::<crate::schema::errors::Result<Vec<TimeSeriesEntry>>>()
            .map_err(|err| err.to_string())?,
    };
    energy_costs_time_series
        .data_points
        .sort_by(|a, b| a.date.cmp(&b.date));

    Ok(Json(energy_costs_time_series))
}
