use rocket::{form::Form, post, FromForm, State};

use crate::{
    client::events::ClientEvent,
    components::{
        component::{Component, ComponentResponse},
        event::EventListenerBuilder, button::ButtonBuilder,
    },
    persistance::{
        electrolyzer::ElectrolyzerClient, simulation::SimulationClient,
        simulation_selection::SimulationSelectionClient,
    },
    responders::htmx_responder::HtmxHeadersBuilder,
    schema::{
        electrolyzer::{
            ElectrolyzerDetailsActions, ElectrolyzerDetailsState, ElectrolyzerDetailsTemplate,
            ElectrolyzerId,
        },
        errors::BannerError,
        user::User,
    },
};

#[derive(FromForm, Debug, Default)]
pub struct SelectElectrolyzerHandlerRequest {
    electrolyzer_id: ElectrolyzerId,
}

#[post("/select_electrolyzer", data = "<request>")]
pub fn select_electrolyzer_handler(
    request: Form<SelectElectrolyzerHandlerRequest>,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
    simulation_client: &State<Box<dyn SimulationClient>>,
    user: User,
    simulation_selection_client: &State<Box<dyn SimulationSelectionClient>>,
) -> ComponentResponse<ElectrolyzerDetailsTemplate, BannerError> {
    let simulation_id = simulation_selection_client.expect_current_selection(user.id())?;
    let mut state = simulation_client.get_simulation_state(&simulation_id)?;
    let electrolyzer = electrolyzer_client.get_electrolyzer(request.electrolyzer_id)?;
    state.electrolyzer_id = electrolyzer.id;
    simulation_client.update(&state)?;

    Component::component(
        HtmxHeadersBuilder::new()
            .trigger(ClientEvent::SelectElectrolyzer)
            .build(),
        ElectrolyzerDetailsTemplate {
            electrolyzer,
            state: ElectrolyzerDetailsState::Selected,
            actions: ElectrolyzerDetailsActions::Selectable,
            list_simulations_listener: EventListenerBuilder::new()
                .event(ClientEvent::ListSimulations)
                .endpoint("/list_electrolyzers")
                .target("#sidebar")
                .build(),
            select_simulation_listener: EventListenerBuilder::new()
                .event(ClientEvent::SelectSimulation)
                .endpoint("/get_selected_electrolyzer")
                .target("#sidebar")
                .build(),
            select_electrolyzer_button: ButtonBuilder::new()
                .endpoint("/select_electrolyzer")
                .target("#sidebar")
                .text("Use")
                .build(),
        },
    )
}
