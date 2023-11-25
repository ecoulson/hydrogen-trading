use rocket::{post, State};

use crate::{
    client::events::ClientEvent,
    components::{
        button::{ButtonBuilder, ButtonVariant},
        component::{Component, ComponentResponse},
        event::EventListenerBuilder,
    },
    persistance::{
        electrolyzer::ElectrolyzerClient, simulation::SimulationClient,
        simulation_selection::SimulationSelectionClient,
    },
    schema::{errors::BannerError, user::User},
    templates::list_electrolyzers_template::{
        ElectrolyzerSearchResultsBuilder, ListElectrolyzersTemplate,
    },
};

#[post("/list_electrolyzers")]
pub fn list_electrolyzers_handler(
    user: User,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
    simulation_client: &State<Box<dyn SimulationClient>>,
    simulation_selection_client: &State<Box<dyn SimulationSelectionClient>>,
) -> ComponentResponse<ListElectrolyzersTemplate, BannerError> {
    let electrolyzers = electrolyzer_client.list_electrolyzers()?;
    let simulation_id = simulation_selection_client.current_selection(user.id())?;
    let select_simulation_listener = EventListenerBuilder::new()
        .event(ClientEvent::SelectSimulation)
        .target("#sidebar")
        .endpoint("/get_selected_electrolyzer")
        .build();
    let list_simulation_listener = EventListenerBuilder::new()
        .event(ClientEvent::ListSimulations)
        .target("#sidebar")
        .endpoint("/list_electrolyzers")
        .build();
    let create_electrolyzer_button = ButtonBuilder::new()
        .variant(ButtonVariant::Primary)
        .endpoint("/create_electrolyzer_form")
        .target("#sidebar")
        .text("Create Electrolyzer")
        .build();

    if simulation_id.is_none() {
        return Component::basic(ListElectrolyzersTemplate {
            search_results: ElectrolyzerSearchResultsBuilder::new()
                .electrolyzers(electrolyzers)
                .build(),
            list_simulation_listener,
            select_simulation_listener,
            create_electrolyzer_button,
        });
    }

    let simulation = simulation_client.get_simulation_state(&simulation_id.unwrap())?;

    Component::basic(ListElectrolyzersTemplate {
        search_results: ElectrolyzerSearchResultsBuilder::new()
            .electrolyzers(electrolyzers)
            .selected_id(simulation.electrolyzer_id)
            .build(),
        list_simulation_listener,
        select_simulation_listener,
        create_electrolyzer_button,
    })
}
