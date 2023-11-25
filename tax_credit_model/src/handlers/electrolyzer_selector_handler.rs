use rocket::{post, State};

use crate::{
    client::{events::ClientEvent, htmx::HtmxSwap},
    components::{
        component::{Component, ComponentResponse},
        event::EventListenerBuilder,
    },
    persistance::{
        electrolyzer::ElectrolyzerClient, simulation::SimulationClient,
        simulation_selection::SimulationSelectionClient,
    },
    schema::{errors::BannerError, user::User},
    templates::list_electrolyzers_template::ElectrolyzerSelectorTemplate,
};

#[post("/electrolyzer_selector")]
pub fn electrolyzer_selector_handler(
    user: User,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
    simulation_client: &State<Box<dyn SimulationClient>>,
    simulation_selection_client: &State<Box<dyn SimulationSelectionClient>>,
) -> ComponentResponse<ElectrolyzerSelectorTemplate, BannerError> {
    let simulation_id = simulation_selection_client.expect_current_selection(user.id())?;
    let simulation = simulation_client.get_simulation_state(&simulation_id)?;

    Component::basic(ElectrolyzerSelectorTemplate {
        selected_id: simulation.electrolyzer_id,
        electrolyzers: electrolyzer_client.list_electrolyzers()?,
        select_electrolyzer_listener: EventListenerBuilder::new()
            .event(ClientEvent::SelectElectrolyzer)
            .endpoint("/electrolyzer_selector")
            .target("#electrolyzer-selector")
            .swap(HtmxSwap::OuterHtml)
            .build(),
    })
}
