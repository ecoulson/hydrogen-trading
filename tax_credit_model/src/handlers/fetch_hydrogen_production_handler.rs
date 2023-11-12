use rocket::{post, serde::json::Json, State};

use crate::{
    persistance::simulation::SimulationClient,
    schema::time_series::{ChartColor, TimeSeries, TimeSeriesEntry},
};

#[post("/fetch_hydrogen_production/<simulation_id>")]
pub fn fetch_hydrogen_production_handler(
    simulation_id: i32,
    simulation_client: &State<Box<dyn SimulationClient>>,
) -> Result<Json<TimeSeries>, String> {
    let simulation = simulation_client
        .get_simulation_state(&simulation_id)
        .map_err(|err| err.to_string())?;
    let hydrogen_production_time_series = TimeSeries {
        label: String::from("Hydrogen Produced"),
        color: ChartColor::Blue,
        data_points: simulation
            .hydrogen_productions
            .iter()
            .map(|production| {
                Ok(TimeSeriesEntry {
                    color: ChartColor::Blue,
                    date: production
                        .production_timestamp
                        .to_utc_date_time()?
                        .to_rfc3339(),
                    value: production.kg_hydrogen,
                })
            })
            .collect::<crate::schema::errors::Result<Vec<TimeSeriesEntry>>>()
            .map_err(|err| err.to_string())?,
    };

    Ok(Json(hydrogen_production_time_series))
}
