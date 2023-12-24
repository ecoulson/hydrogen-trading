use rocket::{post, State};

use crate::{
    components::{
        component::{Component, ComponentResponse},
        electrolyzer::ElectrolyzerList,
        error::BannerError,
    },
    persistance::{
        electrolyzer::ElectrolyzerClient, simulation::SimulationClient,
        simulation_selection::SimulationSelectionClient,
    },
    schema::user::User,
};

#[post("/list_electrolyzers")]
pub fn list_electrolyzers_handler(
    user: User,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
    simulation_client: &State<Box<dyn SimulationClient>>,
    simulation_selection_client: &State<Box<dyn SimulationSelectionClient>>,
) -> ComponentResponse<ElectrolyzerList, BannerError> {
    let electrolyzers = electrolyzer_client.list_electrolyzers()?;
    let simulation_id = simulation_selection_client.current_selection(&user.id)?;

    if simulation_id.is_none() {
        return Component::basic(ElectrolyzerList::render(electrolyzers));
    }

    let simulation = simulation_client.get_simulation_state(&simulation_id.unwrap())?;

    Component::basic(ElectrolyzerList::render_selected(
        simulation.electrolyzer_id,
        electrolyzers,
    ))
}
