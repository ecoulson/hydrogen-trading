use rocket::{post, State};

use crate::{
    components::component::{Component, ComponentResponse},
    persistance::{electrolyzer::ElectrolyzerClient, simulation::SimulationClient},
    schema::{electrolyzer::ElectrolyzerDetailsTemplate, errors::BannerError, user::User},
};

#[post("/get_active_electrolyzer")]
pub fn get_active_electrolyzer_handler(
    user: User,
    simulation_client: &State<Box<dyn SimulationClient>>,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
) -> ComponentResponse<ElectrolyzerDetailsTemplate, BannerError> {
    let simulation = simulation_client.get_simulation_state(&user.simulation_id())?;
    let electrolyzer = electrolyzer_client.get_electrolyzer(simulation.electrolyzer_id)?;

    Component::htmx(ElectrolyzerDetailsTemplate {
        electrolyzer,
        selected: true,
    })
}
