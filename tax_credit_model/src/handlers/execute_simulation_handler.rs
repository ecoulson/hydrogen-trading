use rocket::{form::Form, post, State};

use crate::{
    logic::simulation::simulate,
    persistance::{
        electrolyzer::ElectrolyzerClient, grid::GridClient, simulation::SimulationClient,
    },
    responders::htmx_responder::HtmxTemplate,
    schema::{
        errors::BannerError,
        simulation_schema::{ExecuteSimulationRequest, ExecuteSimulationResponse},
    },
};

#[post("/execute_simulation", data = "<request>")]
pub fn execute_simulation(
    request: Form<ExecuteSimulationRequest>,
    power_grid_fetcher: &State<Box<dyn GridClient>>,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
    simulation_client: &State<Box<dyn SimulationClient>>,
) -> Result<HtmxTemplate<ExecuteSimulationResponse>, HtmxTemplate<BannerError>> {
    let electrolyzer = electrolyzer_client
        .get_electrolyzer(request.electrolyzer_id)
        .map_err(|err| BannerError {
            message: err.to_string(),
        })?;
    let power_grid = power_grid_fetcher
        .get_power_grid()
        .map_err(|err| BannerError {
            message: err.to_string(),
        })?;

    Ok(ExecuteSimulationResponse {
        simulation_result: simulate(
            &power_grid,
            &electrolyzer,
            &request.simulation_time_range,
            simulation_client.inner(),
        )
        .map_err(|err| BannerError {
            message: err.to_string(),
        })?,
    }
    .into())
}
