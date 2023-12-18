use rocket::{post, State};

use crate::{
    components::{
        component::{Component, ComponentResponse},
        electrolyzer::ElectrolyzerSelector,
    },
    persistance::{
        electrolyzer::ElectrolyzerClient, simulation::SimulationClient,
        simulation_selection::SimulationSelectionClient,
    },
    schema::{errors::BannerError, user::User},
};

#[post("/electrolyzer_selector")]
pub fn electrolyzer_selector_handler(
    user: User,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
    simulation_client: &State<Box<dyn SimulationClient>>,
    simulation_selection_client: &State<Box<dyn SimulationSelectionClient>>,
) -> ComponentResponse<ElectrolyzerSelector, BannerError> {
    let simulation_id = simulation_selection_client.expect_current_selection(user.id())?;
    let simulation = simulation_client.get_simulation_state(&simulation_id)?;

    Component::basic(ElectrolyzerSelector::render(
        simulation.electrolyzer_id,
        electrolyzer_client.list_electrolyzers()?,
    ))
}
