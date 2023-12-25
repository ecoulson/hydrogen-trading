use rocket::{form::Form, post, State};

use crate::{
    components::{
        component::{Component, ComponentResponse},
        electrolyzer::ElectrolyzerSelector,
        error::BannerError,
        simulation::{SimulationResultView, SimulationView},
    },
    logic::simulation::{simulate, SimulationState},
    persistance::{
        electrolyzer::ElectrolyzerClient, generation::GenerationClient, grid::GridClient,
        simulation::SimulationClient, simulation_selection::SimulationSelectionClient,
    },
    responders::{client_context::ClientContext, htmx_responder::HtmxHeadersBuilder},
    schema::{simulation::ExecuteSimulationRequest, user::User},
};

#[post("/execute_simulation", data = "<request>")]
pub fn execute_simulation(
    client_context: ClientContext,
    user: User,
    request: Form<ExecuteSimulationRequest>,
    power_grid_fetcher: &State<Box<dyn GridClient>>,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
    simulation_client: &State<Box<dyn SimulationClient>>,
    simulation_selection_client: &State<Box<dyn SimulationSelectionClient>>,
    generation_client: &State<Box<dyn GenerationClient>>,
) -> ComponentResponse<SimulationResultView, BannerError> {
    let mut client_context = client_context;
    let electrolyzer = electrolyzer_client.get_electrolyzer(request.electrolyzer_id)?;
    let power_grid = power_grid_fetcher.get_power_grid()?;
    let current_simulation_id = simulation_selection_client.expect_current_selection(&user.id)?;
    let mut next_simulation = SimulationState::default();
    next_simulation.electrolyzer_id = electrolyzer.id;
    let next_simulation = simulation_client.create_simulation_state(&next_simulation)?;
    let next_url = &format!("simulation/{}", next_simulation.id);
    let location = client_context.mut_location();
    let generation_range = generation_client.get_generation_range()?;
    location.set_path(&next_url);
    simulation_selection_client.select(user.id, next_simulation.id)?;

    Component::component(
        HtmxHeadersBuilder::new()
            .replace_url(&location.build_url())
            .build(),
        SimulationResultView::render(
            SimulationView::render(
                generation_range,
                ElectrolyzerSelector::render(
                    electrolyzer.id,
                    electrolyzer_client.list_electrolyzers()?,
                ),
            ),
            simulate(
                current_simulation_id,
                &power_grid,
                &electrolyzer,
                &request.simulation_time_range,
                simulation_client.inner(),
            )?,
        ),
    )
}
