use rocket::{form::Form, post, State};

use crate::{
    components::{
        component::{Component, ComponentResponse},
        electrolyzer::ElectrolyzerDetails,
    },
    persistance::{
        electrolyzer::ElectrolyzerClient, simulation::SimulationClient,
        simulation_selection::SimulationSelectionClient,
    },
    schema::{electrolyzer::GetElectrolyzerRequest, errors::BannerError, user::User},
};

#[post("/get_electrolyzer", data = "<request>")]
pub fn get_electrolyzer_handler(
    request: Form<GetElectrolyzerRequest>,
    user: User,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
    simulation_client: &State<Box<dyn SimulationClient>>,
    simulation_selection_client: &State<Box<dyn SimulationSelectionClient>>,
) -> ComponentResponse<ElectrolyzerDetails, BannerError> {
    let electrolyzer = electrolyzer_client.get_electrolyzer(request.electrolyzer_id)?;
    let simulation_id = simulation_selection_client.current_selection(user.id())?;

    if simulation_id.is_none() {
        return Component::basic(ElectrolyzerDetails::render_default(electrolyzer));
    }

    let simulation = simulation_client.get_simulation_state(&simulation_id.unwrap())?;

    if request.electrolyzer_id == simulation.electrolyzer_id {
        return Component::basic(ElectrolyzerDetails::render_selected(electrolyzer));
    }

    Component::basic(ElectrolyzerDetails::render_unselected(electrolyzer))
}
