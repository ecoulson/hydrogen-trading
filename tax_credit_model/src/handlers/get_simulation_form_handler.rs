use rocket::{form::Form, post, FromForm, State};

use crate::{
    logic::simulation::SimulationState,
    persistance::{electrolyzer::ElectrolyzerClient, simulation::SimulationClient},
    responders::htmx_responder::{HtmxHeadersBuilder, HtmxTemplate},
    schema::{errors::BannerError, time::DateTimeRange},
    templates::{
        list_electrolyzers_template::ElectrolyzerSelectorTemplate,
        simulation_form_template::SimulationFormTemplate,
    },
};

#[derive(FromForm)]
pub struct SimulationFormRequest {
    simulation_id: i32,
}

#[post("/simulation_form", data = "<request>")]
pub fn get_simulation_form_handler(
    request: Form<SimulationFormRequest>,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
    simulation_client: &State<Box<dyn SimulationClient>>,
) -> Result<HtmxTemplate<SimulationFormTemplate>, HtmxTemplate<BannerError>> {
    let electrolyzers = electrolyzer_client
        .list_electrolyzers()
        .map_err(BannerError::create_from_error)?;

    // Simulations require an electrolyzer to exist. Somehow want a better solution
    if electrolyzers.is_empty() {
        return Err(BannerError::create_from_message("No electrolyzers exist"));
    }

    let mut simulation_state = SimulationState::default();
    simulation_state.electrolyzer_id = electrolyzers[0].id;
    let simulation_state = simulation_client
        .ensure_simulation_exists(&request.simulation_id)
        .map_err(BannerError::create_from_error)?;

    Ok(HtmxTemplate::new(
        HtmxHeadersBuilder::new().build(),
        SimulationFormTemplate {
            generation_range: DateTimeRange {
                start: String::from("2023-01-01T00:00"),
                end: String::from("2023-07-31T23:59"),
            },
            electrolyzer_selector: ElectrolyzerSelectorTemplate {
                electrolyzers,
                selected_id: simulation_state.electrolyzer_id,
                simulation_id: simulation_state.id,
            },
            simulation_id: simulation_state.id,
        },
    ))
}
