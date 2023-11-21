use rocket::{post, serde::json::Json, State};

use crate::{
    persistance::simulation::SimulationClient,
    schema::histogram::{HistogramData, HistogramDataset, Labels},
};

#[post("/fetch_hourly_histogram/<simulation_id>")]
pub fn fetch_hourly_histogram_handler(
    simulation_id: i32,
    simulation_client: &State<Box<dyn SimulationClient>>,
) -> Json<HistogramData> {
    if let Ok(simulation) = simulation_client.get_simulation_state(&simulation_id) {
        Json(HistogramData {
            labels: Labels {
                x: String::from("Tax Credit Tier"),
                y: String::from("Quarter Hours"),
            },
            keys: vec![
                String::from("0%"),
                String::from("20%"),
                String::from("25%"),
                String::from("33%"),
                String::from("100%"),
            ],
            datasets: vec![HistogramDataset {
                label: String::from("Credit Breakdown"),
                data_points: vec![
                    simulation.tax_credit_summary.credit_hours_none,
                    simulation.tax_credit_summary.credit_hours_20,
                    simulation.tax_credit_summary.credit_hours_25,
                    simulation.tax_credit_summary.credit_hours_33,
                    simulation.tax_credit_summary.credit_hours_full,
                ],
            }],
        })
    } else {
        Json(HistogramData {
            labels: Labels {
                x: String::from("Tax Credit Tier"),
                y: String::from("Quarter Hours"),
            },
            keys: vec![],
            datasets: vec![],
        })
    }
}
