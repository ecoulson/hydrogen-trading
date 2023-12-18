use rocket::{form::Form, post, FromForm, State};

use crate::{
    components::{
        component::{Component, ComponentResponse},
        electrolyzer::ElectrolyzerSearchResults,
    },
    persistance::{
        electrolyzer::ElectrolyzerClient, simulation::SimulationClient,
        simulation_selection::SimulationSelectionClient,
    },
    schema::{errors::BannerError, user::User, electrolyzer::SearchElectrolyzersRequest},
};

#[post("/search_electrolyzers", data = "<request>")]
pub fn search_electrolyzers_handler(
    user: User,
    request: Form<SearchElectrolyzersRequest>,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
    simulation_client: &State<Box<dyn SimulationClient>>,
    simulation_selection_client: &State<Box<dyn SimulationSelectionClient>>,
) -> ComponentResponse<ElectrolyzerSearchResults, BannerError> {
    let simulation_id = simulation_selection_client.current_selection(user.id())?;
    let simulation = simulation_client.get_simulation_state(&simulation_id.unwrap())?;
    let results = electrolyzer_client
        .list_electrolyzers()?
        .into_iter()
        .filter(|electrolyzer| {
            if request.query.trim().is_empty() {
                return true;
            }

            electrolyzer
                .name
                .to_lowercase()
                .starts_with(&request.query.to_lowercase())
        })
        .collect();

    if simulation_id.is_none() {
        return Component::basic(ElectrolyzerSearchResults::render(results));
    }

    Component::basic(ElectrolyzerSearchResults::render_selected(
        simulation.electrolyzer_id,
        results,
    ))
}
