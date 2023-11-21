use rocket::{form::Form, post, FromForm, State};

use crate::{
    persistance::{electrolyzer::ElectrolyzerClient, simulation::SimulationClient},
    responders::{htmx_responder::HtmxTemplate, user_context::UserContext},
    schema::errors::BannerError,
    templates::list_electrolyzers_template::ElectrolyzerSearchResults,
};

#[derive(FromForm)]
pub struct SearchElectrolyzersRequest {
    query: String,
}

#[post("/search_electrolyzers", data = "<request>")]
pub fn search_electrolyzers_handler(
    user_context: UserContext,
    request: Form<SearchElectrolyzersRequest>,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
    simulation_client: &State<Box<dyn SimulationClient>>,
) -> Result<HtmxTemplate<ElectrolyzerSearchResults>, HtmxTemplate<BannerError>> {
    let user = user_context
        .user()
        .ok_or_else(|| BannerError::create_from_message("User not logged in"))?;
    // This will throw in some cases
    // Will eventually convert to simulation_client.get_active_simulation(user.id()) -> Option<Simulation>
    let simulation = simulation_client
        .get_simulation_state(&user.simulation_id())
        .map_err(BannerError::create_from_error)?;

    if request.query.trim().is_empty() {
        return Ok(ElectrolyzerSearchResults {
            electrolyzers: electrolyzer_client
                .list_electrolyzers()
                .map_err(BannerError::create_from_error)?,
            selected_id: Some(simulation.electrolyzer_id),
        }
        .into());
    }

    Ok(ElectrolyzerSearchResults {
        selected_id: Some(simulation.electrolyzer_id),
        electrolyzers: electrolyzer_client
            .list_electrolyzers()
            .map_err(BannerError::create_from_error)?
            .into_iter()
            .filter(|electrolyzer| {
                electrolyzer
                    .name
                    .to_lowercase()
                    .starts_with(&request.query.to_lowercase())
            })
            .collect(),
    }
    .into())
}
