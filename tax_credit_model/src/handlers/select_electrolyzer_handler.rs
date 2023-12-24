use rocket::{form::Form, post, State};

use crate::{
    client::events::ClientEvent,
    components::{
        component::{Component, ComponentResponse},
        electrolyzer::ElectrolyzerDetails,
        error::BannerError,
    },
    persistance::{
        electrolyzer::ElectrolyzerClient, simulation::SimulationClient,
        simulation_selection::SimulationSelectionClient,
    },
    responders::htmx_responder::HtmxHeadersBuilder,
    schema::{electrolyzer::SelectElectrolyzerHandlerRequest, user::User},
};

#[post("/select_electrolyzer", data = "<request>")]
pub fn select_electrolyzer_handler(
    request: Form<SelectElectrolyzerHandlerRequest>,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
    simulation_client: &State<Box<dyn SimulationClient>>,
    user: User,
    simulation_selection_client: &State<Box<dyn SimulationSelectionClient>>,
) -> ComponentResponse<ElectrolyzerDetails, BannerError> {
    let simulation_id = simulation_selection_client.expect_current_selection(&user.id)?;
    let mut state = simulation_client.get_simulation_state(&simulation_id)?;
    let electrolyzer = electrolyzer_client.get_electrolyzer(request.electrolyzer_id)?;
    state.electrolyzer_id = electrolyzer.id;
    simulation_client.update(&state)?;

    Component::component(
        HtmxHeadersBuilder::new()
            .trigger(ClientEvent::SelectElectrolyzer)
            .build(),
        ElectrolyzerDetails::render_selected(electrolyzer),
    )
}
