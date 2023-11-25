use rocket::{form::Form, post, FromForm, State};

use crate::{
    client::events::ClientEvent,
    components::{
        button::ButtonBuilder,
        component::{Component, ComponentResponse},
        event::EventListenerBuilder,
    },
    persistance::{
        electrolyzer::ElectrolyzerClient, simulation::SimulationClient,
        simulation_selection::SimulationSelectionClient,
    },
    schema::{
        electrolyzer::{
            ElectrolyzerDetailsActions, ElectrolyzerDetailsState, ElectrolyzerDetailsTemplate,
            ElectrolyzerId,
        },
        errors::BannerError,
        user::User,
    },
};

#[derive(FromForm)]
pub struct GetElectrolyzerRequest {
    pub electrolyzer_id: ElectrolyzerId,
}

#[post("/get_electrolyzer", data = "<request>")]
pub fn get_electrolyzer_handler(
    request: Form<GetElectrolyzerRequest>,
    user: User,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
    simulation_client: &State<Box<dyn SimulationClient>>,
    simulation_selection_client: &State<Box<dyn SimulationSelectionClient>>,
) -> ComponentResponse<ElectrolyzerDetailsTemplate, BannerError> {
    let electrolyzer = electrolyzer_client.get_electrolyzer(request.electrolyzer_id)?;
    let simulation_id = simulation_selection_client.current_selection(user.id())?;
    let list_simulations_listener = EventListenerBuilder::new()
        .event(ClientEvent::ListSimulations)
        .endpoint("/list_electrolyzers")
        .target("#sidebar")
        .build();
    let select_simulation_listener = EventListenerBuilder::new()
        .event(ClientEvent::SelectSimulation)
        .endpoint("/get_selected_electrolyzer")
        .target("#sidebar")
        .build();

    if simulation_id.is_none() {
        return Component::basic(ElectrolyzerDetailsTemplate {
            electrolyzer,
            actions: ElectrolyzerDetailsActions::None,
            state: ElectrolyzerDetailsState::Default,
            list_simulations_listener,
            select_simulation_listener,
            select_electrolyzer_button: ButtonBuilder::new()
                .endpoint("/select_electrolyzer")
                .target("#sidebar")
                .disabled()
                .text("Use")
                .build(),
        });
    }

    let simulation = simulation_client.get_simulation_state(&simulation_id.unwrap())?;

    if simulation.electrolyzer_id == request.electrolyzer_id {}

    Component::basic(ElectrolyzerDetailsTemplate {
        electrolyzer,
        state: simulation.electrolyzer_state(&request.electrolyzer_id),
        actions: ElectrolyzerDetailsActions::Selectable,
        list_simulations_listener,
        select_simulation_listener,
        select_electrolyzer_button: ButtonBuilder::new()
            .endpoint("/select_electrolyzer")
            .target("#sidebar")
            .text("Use")
            .build(),
    })
}
