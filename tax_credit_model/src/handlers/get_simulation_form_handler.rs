use rocket::{post, State};

use crate::{
    components::component::{Component, ComponentResponse},
    logic::simulation::SimulationState,
    persistance::{
        electrolyzer::ElectrolyzerClient, simulation::SimulationClient,
        simulation_selection::SimulationSelectionClient,
    },
    responders::htmx_responder::HtmxHeadersBuilder,
    schema::{errors::BannerError, time::DateTimeRange, user::User},
    templates::{
        list_electrolyzers_template::ElectrolyzerSelectorTemplate,
        simulation_form_template::SimulationFormTemplate,
    },
};

#[post("/simulation_form")]
pub fn get_simulation_form_handler(
    user: User,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
    simulation_client: &State<Box<dyn SimulationClient>>,
    simulation_selection_client: &State<Box<dyn SimulationSelectionClient>>,
) -> ComponentResponse<SimulationFormTemplate, BannerError> {
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
        SimulationFormTemplate {
            generation_range: DateTimeRange {
                start: String::from("2023-01-01T00:00"),
                end: String::from("2023-07-31T23:59"),
            },
            electrolyzer_selector: ElectrolyzerSelectorTemplate {
                electrolyzers,
                selected_id: simulation_state.electrolyzer_id,
            },
        },
    )
}
