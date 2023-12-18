use rocket::{post, State};

use crate::{
    components::{
        component::{Component, ComponentResponse},
        electrolyzer::ElectrolyzerSelector,
        simulation::SimulationView,
    },
    logic::simulation::SimulationState,
    persistance::{
        electrolyzer::ElectrolyzerClient, simulation::SimulationClient,
        simulation_selection::SimulationSelectionClient,
    },
    responders::htmx_responder::HtmxHeadersBuilder,
    schema::{errors::BannerError, time::DateTimeRange, user::User},
};

#[post("/get_selected_simulation")]
pub fn get_selected_simulation_handler(
    user: User,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
    simulation_client: &State<Box<dyn SimulationClient>>,
    simulation_selection_client: &State<Box<dyn SimulationSelectionClient>>,
) -> ComponentResponse<SimulationView, BannerError> {
    let electrolyzers = electrolyzer_client.list_electrolyzers()?;

    if electrolyzers.is_empty() {
        return Err(BannerError::create_from_message("No electrolyzers exist"));
    }

    let mut simulation_state = SimulationState::default();
    simulation_state.electrolyzer_id = electrolyzers[0].id;
    let simulation_id = simulation_selection_client.expect_current_selection(user.id())?;
    let simulation_state = simulation_client.get_simulation_state(&simulation_id)?;

    Component::component(
        HtmxHeadersBuilder::new().build(),
        SimulationView::render(
            DateTimeRange {
                start: String::from("2023-01-01T00:00"),
                end: String::from("2023-07-31T23:59"),
            },
            ElectrolyzerSelector::render(simulation_state.electrolyzer_id, electrolyzers),
        ),
    )
}
