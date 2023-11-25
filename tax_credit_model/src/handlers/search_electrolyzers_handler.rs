use rocket::{form::Form, post, FromForm, State};

use crate::{
    components::component::{Component, ComponentResponse},
    persistance::{
        electrolyzer::ElectrolyzerClient, simulation::SimulationClient,
        simulation_selection::SimulationSelectionClient,
    },
    schema::{errors::BannerError, user::User},
    templates::list_electrolyzers_template::{
        ElectrolyzerSearchResults, ElectrolyzerSearchResultsBuilder,
    },
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
    let simulation_id = simulation_selection_client.current_selection(user.id())?;

    if simulation_id.is_none() {
        return Component::basic(ElectrolyzerSearchResultsBuilder::new().build());
    }

    let simulation = simulation_client.get_simulation_state(&simulation_id.unwrap())?;

    if request.query.trim().is_empty() {
        return Component::basic(
            ElectrolyzerSearchResultsBuilder::new()
                .electrolyzers(electrolyzer_client.list_electrolyzers()?)
                .selected_id(simulation.electrolyzer_id)
                .build(),
        );
    }

    let filtered_electrolyzers = electrolyzer_client
        .list_electrolyzers()?
        .into_iter()
        .filter(|electrolyzer| {
            electrolyzer
                .name
                .to_lowercase()
                .starts_with(&request.query.to_lowercase())
        })
        .collect();

    Component::basic(
        ElectrolyzerSearchResultsBuilder::new()
            .selected_id(simulation.electrolyzer_id)
            .electrolyzers(filtered_electrolyzers)
            .build(),
    )
}
