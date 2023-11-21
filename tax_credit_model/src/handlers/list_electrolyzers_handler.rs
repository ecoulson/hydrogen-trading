use rocket::{post, State};

use crate::{
    persistance::{electrolyzer::ElectrolyzerClient, simulation::SimulationClient},
    responders::{htmx_responder::HtmxTemplate, user_context::UserContext},
    schema::errors::BannerError,
    templates::list_electrolyzers_template::{
        ElectrolyzerSearchResults, ListElectrolyzersTemplate,
    },
};

#[post("/list_electrolyzers")]
pub fn list_electrolyzers_handler(
    user_context: UserContext,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
    simulation_client: &State<Box<dyn SimulationClient>>,
) -> Result<HtmxTemplate<ListElectrolyzersTemplate>, HtmxTemplate<BannerError>> {
    let user = user_context
        .user()
        .ok_or_else(|| BannerError::create_from_message("User not logged in"))?;
    let simulation = simulation_client
        .get_simulation_state(&user.simulation_id())
        .map_err(BannerError::create_from_error)?;
    let electrolyzers = electrolyzer_client
        .list_electrolyzers()
        .map_err(BannerError::create_from_error)?;

    Ok(ListElectrolyzersTemplate {
        search_results: ElectrolyzerSearchResults {
            electrolyzers,
            selected_id: Some(simulation.electrolyzer_id),
        },
    }
    .into())
}
