use rocket::{form::Form, post, FromForm, State};

use crate::{
    components::component::{Component, ComponentResponse},
    persistance::{electrolyzer::ElectrolyzerClient, simulation::SimulationClient},
    responders::htmx_responder::HtmxHeadersBuilder,
    schema::{electrolyzer::ElectrolyzerDetailsTemplate, errors::BannerError, user::User},
};

#[derive(FromForm, Debug, Default)]
pub struct SelectElectrolyzerHandlerRequest {
    electrolyzer_id: usize,
}

#[post("/select_electrolyzer", data = "<request>")]
pub fn select_electrolyzer_handler(
    request: Form<SelectElectrolyzerHandlerRequest>,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
    simulation_client: &State<Box<dyn SimulationClient>>,
    user: User,
) -> ComponentResponse<ElectrolyzerDetailsTemplate, BannerError> {
    let mut state = simulation_client.get_simulation_state(&user.simulation_id())?;
    let electrolyzer = electrolyzer_client.get_electrolyzer(request.electrolyzer_id)?;
    state.electrolyzer_id = electrolyzer.id;
    simulation_client.update(&state)?;

    Component::component(
        HtmxHeadersBuilder::new()
            .trigger("electrolyzer-selected")
            .build(),
        ElectrolyzerDetailsTemplate {
            electrolyzer,
            selected: true,
        },
    )
}
