use rocket::{form::Form, post, State};

use crate::{
    logic::simulation::SimulationState,
    persistance::{
        electrolyzer::ElectrolyzerClient, simulation::SimulationClient, user::UserClient,
    },
    responders::{
        client_context::ClientContext, htmx_responder::HtmxTemplate, user_context::UserContext,
    },
    schema::errors::BannerError,
    templates::simulation_form_template::SimulationFormTemplate,
};

use super::select_simulation_handler::{select_simulation_handler, SelectSimulationRequest};

#[post("/initialize_simulation")]
pub fn initialize_simulation_handler(
    user_context: UserContext,
    client_context: ClientContext,
    user_client: &State<Box<dyn UserClient>>,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
    simulation_client: &State<Box<dyn SimulationClient>>,
) -> Result<HtmxTemplate<SimulationFormTemplate>, HtmxTemplate<BannerError>> {
    let electrolyzers = electrolyzer_client
        .list_electrolyzers()
        .map_err(BannerError::create_from_error)?;

    if electrolyzers.is_empty() {
        return Err(BannerError::create_from_message("No electrolyzers exist"));
    }

    let simulation = simulation_client
        .create_simulation_state(&SimulationState::default())
        .map_err(BannerError::create_from_error)?;

    select_simulation_handler(
        Form::from(SelectSimulationRequest {
            simulation_id: simulation.id,
        }),
        user_context,
        user_client,
        client_context,
        simulation_client,
        electrolyzer_client,
    )
}
