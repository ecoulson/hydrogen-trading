use rocket::{post, State};

use crate::{
    components::component::{Component, ComponentResponse},
    persistance::{electrolyzer::ElectrolyzerClient, simulation::SimulationClient},
    schema::{errors::BannerError, user::User},
    templates::list_electrolyzers_template::{
        ElectrolyzerSearchResults, ListElectrolyzersTemplate,
    },
};

#[post("/list_electrolyzers")]
pub fn list_electrolyzers_handler(
    user: User,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
    simulation_client: &State<Box<dyn SimulationClient>>,
) -> ComponentResponse<ListElectrolyzersTemplate, BannerError> {
    let simulation = simulation_client.get_simulation_state(&user.simulation_id())?;
    let electrolyzers = electrolyzer_client.list_electrolyzers()?;

    Component::htmx(ListElectrolyzersTemplate {
        search_results: ElectrolyzerSearchResults {
            electrolyzers,
            selected_id: Some(simulation.electrolyzer_id),
        },
    })
}
