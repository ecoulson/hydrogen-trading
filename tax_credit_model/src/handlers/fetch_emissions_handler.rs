use rocket::{get, serde::json::Json, State};

use crate::{
    persistance::simulation::SimulationClient,
    schema::{
        simulation_schema::TaxCredit45VTier,
        time_series::{ChartColor, TimeSeries, TimeSeriesEntry},
    },
};

#[get("/fetch_emissions/<simulation_id>")]
pub fn fetch_emissions_handler(
    simulation_id: i32,
    simulation_client: &State<Box<dyn SimulationClient>>,
) -> Result<Json<TimeSeries>, String> {
    let simulation = simulation_client
        .get_simulation_state(&simulation_id)
        .map_err(|err| err.to_string())?;
    let data = simulation
        .emissions
        .iter()
        .zip(simulation.tax_credit)
        .map(|(emission, tax_credit)| {
            Ok(TimeSeriesEntry {
                date: emission.emission_timestamp.to_utc_date_time()?.to_rfc3339(),
                value: emission.amount_emitted_kg,
                color: match tax_credit.tier {
                    TaxCredit45VTier::Max => ChartColor::Green,
                    TaxCredit45VTier::Tier1 => ChartColor::Chartreuse,
                    TaxCredit45VTier::Tier2 => ChartColor::Yellow,
                    TaxCredit45VTier::Tier3 => ChartColor::Orange,
                    TaxCredit45VTier::None => ChartColor::Red,
                },
            })
        })
        .collect::<crate::schema::errors::Result<Vec<TimeSeriesEntry>>>()
        .map_err(|err| err.to_string())?;

    Ok(Json(TimeSeries {
        color: ChartColor::Blue,
        label: String::from("emissions (kg CO2)"),
        data_points: data,
    }))
}
