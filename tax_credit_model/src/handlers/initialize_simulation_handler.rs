use rocket::{form::Form, post, State};

use crate::{
    components::component::ComponentResponse,
    logic::simulation::SimulationState,
    persistance::{
        electrolyzer::ElectrolyzerClient, simulation::SimulationClient, user::UserClient,
    },
    responders::client_context::ClientContext,
    schema::{errors::BannerError, user::User},
    templates::simulation_form_template::SimulationFormTemplate,
};

use super::select_simulation_handler::{select_simulation_handler, SelectSimulationRequest};

#[post("/initialize_simulation")]
pub fn initialize_simulation_handler(
    user: User,
    client_context: ClientContext,
    user_client: &State<Box<dyn UserClient>>,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
    simulation_client: &State<Box<dyn SimulationClient>>,
) -> ComponentResponse<SimulationFormTemplate, BannerError> {
    let electrolyzers = electrolyzer_client.list_electrolyzers()?;

    if electrolyzers.is_empty() {
        return Err(BannerError::create_from_message("No electrolyzers exist"));
    }

    let simulation = simulation_client.create_simulation_state(&SimulationState::default())?;

    select_simulation_handler(
        Form::from(SelectSimulationRequest {
            simulation_id: simulation.id,
        }),
        user,
        user_client,
        client_context,
        simulation_client,
        electrolyzer_client,
    )
}
