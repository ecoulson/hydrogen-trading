use rocket::{post, State};

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
    schema::{
        electrolyzer::{
            ElectrolyzerDetailsActions, ElectrolyzerDetailsState, ElectrolyzerDetailsTemplate,
        },
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
) -> ComponentResponse<ElectrolyzerDetailsTemplate, BannerError> {
    let simulation_id = simulation_selection_client.expect_current_selection(user.id())?;
    let simulation = simulation_client.get_simulation_state(&simulation_id)?;
    let electrolyzer = electrolyzer_client.get_electrolyzer(simulation.electrolyzer_id)?;

    Component::basic(ElectrolyzerDetailsTemplate {
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
    })
}
