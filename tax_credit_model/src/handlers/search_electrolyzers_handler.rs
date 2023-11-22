use rocket::{form::Form, post, FromForm, State};

use crate::{
    components::component::{Component, ComponentResponse},
    persistance::{
        electrolyzer::ElectrolyzerClient, simulation::SimulationClient,
        simulation_selection::SimulationSelectionClient,
    },
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
    simulation_selection_client: &State<Box<dyn SimulationSelectionClient>>,
) -> ComponentResponse<ElectrolyzerSearchResults, BannerError> {
    let simulation_id = simulation_selection_client.expect_current_selection(user.id())?;
    let simulation = simulation_client.get_simulation_state(&simulation_id)?;

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
