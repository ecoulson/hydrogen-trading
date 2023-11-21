use rocket::{form::Form, post, State};

use crate::{
    components::component::{Component, ComponentResponse},
    logic::simulation::{simulate, SimulationState},
    persistance::{
        electrolyzer::ElectrolyzerClient, grid::GridClient, simulation::SimulationClient,
        user::UserClient,
    },
    responders::{client_context::ClientContext, htmx_responder::HtmxHeadersBuilder},
    schema::{
        errors::BannerError,
        simulation_schema::{ExecuteSimulationRequest, ExecuteSimulationResponse},
        time::DateTimeRange,
        user::User,
    },
    templates::{
        list_electrolyzers_template::ElectrolyzerSelectorTemplate,
        simulation_form_template::SimulationFormTemplate,
    },
};

#[post("/execute_simulation", data = "<request>")]
pub fn execute_simulation(
    client_context: ClientContext,
    user: User,
    request: Form<ExecuteSimulationRequest>,
    power_grid_fetcher: &State<Box<dyn GridClient>>,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
    simulation_client: &State<Box<dyn SimulationClient>>,
    user_client: &State<Box<dyn UserClient>>,
) -> ComponentResponse<ExecuteSimulationResponse, BannerError> {
    let mut user = user;
    let mut client_context = client_context;
    let electrolyzer = electrolyzer_client.get_electrolyzer(request.electrolyzer_id)?;
    let power_grid = power_grid_fetcher.get_power_grid()?;
    let current_simulation_id = user.simulation_id();
    let mut next_simulation = SimulationState::default();
    next_simulation.electrolyzer_id = electrolyzer.id;
    let next_simulation = simulation_client.create_simulation_state(&next_simulation)?;
    let next_url = &format!("simulation/{}", next_simulation.id);
    let location = client_context.mut_location();
    location.set_path(&next_url);
    user.set_simulation_id(next_simulation.id);
    user_client.update_user(&user)?;

    Component::component(
        HtmxHeadersBuilder::new()
            .replace_url(&location.build_url())
            .build(),
        ExecuteSimulationResponse {
            simulation_form: SimulationFormTemplate {
                generation_range: DateTimeRange::default(),
                electrolyzer_selector: ElectrolyzerSelectorTemplate {
                    electrolyzers: electrolyzer_client.list_electrolyzers()?,
                    selected_id: electrolyzer.id,
                },
            },
            simulation_result: simulate(
                current_simulation_id,
                &power_grid,
                &electrolyzer,
                &request.simulation_time_range,
                simulation_client.inner(),
            )?,
        },
    )
}
