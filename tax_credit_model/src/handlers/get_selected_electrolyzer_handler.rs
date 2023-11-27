use rocket::{post, State};

use crate::{
    components::component::{Component, ComponentResponse},
    persistance::{
        electrolyzer::ElectrolyzerClient, simulation::SimulationClient,
        simulation_selection::SimulationSelectionClient,
    },
    schema::{
        electrolyzer::{ElectrolyzerDetails, ElectrolyzerDetailsBuilder},
        errors::BannerError,
        user::User,
    },
};

#[post("/get_selected_electrolyzer")]
pub fn get_selected_electrolyzer_handler(
    user: User,
    simulation_client: &State<Box<dyn SimulationClient>>,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
    simulation_selection_client: &State<Box<dyn SimulationSelectionClient>>,
) -> ComponentResponse<ElectrolyzerDetails, BannerError> {
    let simulation_id = simulation_selection_client.expect_current_selection(user.id())?;
    let simulation = simulation_client.get_simulation_state(&simulation_id)?;
    let electrolyzer = electrolyzer_client.get_electrolyzer(simulation.electrolyzer_id)?;

    Component::basic(
        ElectrolyzerDetailsBuilder::new()
            .electrolyzer(electrolyzer)
            .selected()
            .build(),
    )
}
