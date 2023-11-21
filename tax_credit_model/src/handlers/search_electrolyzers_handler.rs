use rocket::{form::Form, post, FromForm, State};

use crate::{
    components::component::{Component, ComponentResponse},
    persistance::{electrolyzer::ElectrolyzerClient, simulation::SimulationClient},
    responders::htmx_responder::HtmxTemplate,
    schema::{errors::BannerError, user::User},
    templates::list_electrolyzers_template::ElectrolyzerSearchResults,
};

#[derive(FromForm)]
pub struct SearchElectrolyzersRequest {
    query: String,
}

#[post("/search_electrolyzers", data = "<request>")]
pub fn search_electrolyzers_handler(
    user: User,
    request: Form<SearchElectrolyzersRequest>,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
    simulation_client: &State<Box<dyn SimulationClient>>,
) -> ComponentResponse<ElectrolyzerSearchResults, BannerError> {
    // This will throw in some cases because simulation id may not exist
    // Will eventually convert to simulation_client.get_active_simulation(user.id()) -> Option<Simulation>
    let simulation = simulation_client.get_simulation_state(&user.simulation_id())?;

    if request.query.trim().is_empty() {
        return Ok(HtmxTemplate::template(ElectrolyzerSearchResults {
            electrolyzers: electrolyzer_client.list_electrolyzers()?,
            selected_id: Some(simulation.electrolyzer_id),
        }));
    }

    Component::basic(ElectrolyzerSearchResults {
        selected_id: Some(simulation.electrolyzer_id),
        electrolyzers: electrolyzer_client
            .list_electrolyzers()?
            .into_iter()
            .filter(|electrolyzer| {
                electrolyzer
                    .name
                    .to_lowercase()
                    .starts_with(&request.query.to_lowercase())
            })
            .collect(),
    })
}
