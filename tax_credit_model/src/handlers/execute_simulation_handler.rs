use rocket::{form::Form, post, State};

use crate::{
    logic::{grid_fetcher::GridFetcher, simulation::simulate},
    persistance::electrolyzer::ElectrolyzerPersistanceClient,
    responders::htmx_responder::HtmxTemplate,
    schema::simulation_schema::{ExecuteSimulationRequest, ExecuteSimulationResponse},
};

#[post("/execute_simulation", data = "<request>")]
pub async fn execute_simulation(
    request: Form<ExecuteSimulationRequest>,
    power_grid_fetcher: &State<Box<dyn GridFetcher>>,
    electrolyzer_client: &State<Box<dyn ElectrolyzerPersistanceClient>>,
) -> HtmxTemplate<ExecuteSimulationResponse> {
    let electrolyzer = electrolyzer_client
        .get_electrolyzer(request.electrolyzer_id)
        .expect("Electrolyzer should exist");
    let power_grid = power_grid_fetcher.get_power_grid();

    ExecuteSimulationResponse {
        simulation_result:simulate(&power_grid, &electrolyzer, &request.simulation_time_range),
    }
    .into()
}
