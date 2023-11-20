use rocket::{form::Form, post, State};

use crate::{
    logic::simulation::{simulate, SimulationState},
    persistance::{
        electrolyzer::ElectrolyzerClient, grid::GridClient, simulation::SimulationClient,
        user::UserClient,
    },
    responders::{
        client_context::ClientContext,
        htmx_responder::{HtmxHeadersBuilder, HtmxTemplate},
        user_context::UserContext,
    },
    schema::{
        errors::BannerError,
        simulation_schema::{ExecuteSimulationRequest, ExecuteSimulationResponse},
        time::DateTimeRange,
    },
    templates::{
        list_electrolyzers_template::ElectrolyzerSelectorTemplate,
        simulation_form_template::SimulationFormTemplate,
    },
};

#[post("/execute_simulation", data = "<request>")]
pub fn execute_simulation(
    client_context: ClientContext,
    user_context: UserContext,
    request: Form<ExecuteSimulationRequest>,
    power_grid_fetcher: &State<Box<dyn GridClient>>,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
    simulation_client: &State<Box<dyn SimulationClient>>,
    user_client: &State<Box<dyn UserClient>>,
) -> Result<HtmxTemplate<ExecuteSimulationResponse>, HtmxTemplate<BannerError>> {
    let mut user_context = user_context;
    let user = user_context
        .user_mut()
        .ok_or_else(|| BannerError::create_from_message("User not logged in"))?;
    let mut client_context = client_context;
    let electrolyzer = electrolyzer_client
        .get_electrolyzer(request.electrolyzer_id)
        .map_err(BannerError::create_from_error)?;
    let power_grid = power_grid_fetcher
        .get_power_grid()
        .map_err(BannerError::create_from_error)?;
    let mut next_simulation = SimulationState::default();
    next_simulation.electrolyzer_id = electrolyzer.id;
    let next_simulation = simulation_client
        .create_simulation_state(&next_simulation)
        .map_err(BannerError::create_from_error)?;
    let next_url = &format!("simulation/{}", next_simulation.id);
    let location = client_context.mut_location();
    location.set_path(&next_url);
    user.set_simulation_id(next_simulation.id);
    user_client
        .update_user(user)
        .map_err(BannerError::create_from_error)?;

    Ok(HtmxTemplate::new(
        HtmxHeadersBuilder::new()
            .replace_url(&location.build_url())
            .build(),
        ExecuteSimulationResponse {
            simulation_form: SimulationFormTemplate {
                generation_range: DateTimeRange::default(),
                electrolyzer_selector: ElectrolyzerSelectorTemplate {
                    electrolyzers: electrolyzer_client
                        .list_electrolyzers()
                        .map_err(BannerError::create_from_error)?,
                    selected_id: electrolyzer.id,
                },
            },
            simulation_result: simulate(
                user.simulation_id(),
                &power_grid,
                &electrolyzer,
                &request.simulation_time_range,
                simulation_client.inner(),
            )
            .map_err(BannerError::create_from_error)?,
        },
    ))
}
