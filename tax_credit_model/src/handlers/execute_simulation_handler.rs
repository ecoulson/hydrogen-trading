use rocket::{form::Form, post, State};

use crate::{
    logic::{grid_fetcher::GridFetcher, simulation::simulate},
    persistance::{
        db::{DatabaseClient, DatabaseConnectionParameters},
        electrolyzer::ElectrolyzerPersistanceClient,
    },
    responders::htmx_responder::HtmxTemplate,
    schema::{
        errors::BannerError,
        simulation_schema::{ExecuteSimulationRequest, ExecuteSimulationResponse},
    },
};

#[post("/execute_simulation", data = "<request>")]
pub async fn execute_simulation(
    request: Form<ExecuteSimulationRequest>,
    power_grid_fetcher: &State<Box<dyn GridFetcher>>,
    electrolyzer_client: &State<Box<dyn ElectrolyzerPersistanceClient>>,
) -> Result<HtmxTemplate<ExecuteSimulationResponse>, HtmxTemplate<BannerError>> {
    let mut client = DatabaseClient::open(&DatabaseConnectionParameters::new(
        "hydrogen_trading_dev",
        "localhost",
        "hydrogen_trading",
    ))
    .await
    .map_err(|err| BannerError {
        message: err.to_string(),
    })?;
    let electrolyzer = electrolyzer_client
        .get_electrolyzer(request.electrolyzer_id)
        .map_err(|err| BannerError {
            message: err.to_string(),
        })?;
    let power_grid = power_grid_fetcher
        .get_power_grid(&mut client)
        .await
        .map_err(|err| BannerError {
            message: err.to_string(),
        })?;

    Ok(ExecuteSimulationResponse {
        simulation_result: simulate(&power_grid, &electrolyzer, &request.simulation_time_range)
            .map_err(|err| BannerError {
                message: err.to_string(),
            })?,
    }
    .into())
}
