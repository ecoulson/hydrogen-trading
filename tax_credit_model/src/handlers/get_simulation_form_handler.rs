use rocket::{post, State};

use crate::{
    persistance::{electrolyzer::ElectrolyzerClient, simulation::SimulationClient},
    responders::{context::Context, htmx_responder::HtmxTemplate},
    schema::{errors::BannerError, time::DateTimeRange},
    templates::{
        list_electrolyzers_template::ElectrolyzerSelectorTemplate,
        simulation_form_template::SimulationFormTemplate,
    },
};

#[post("/simulation_form")]
pub fn get_simulation_form_handler(
    context: Context,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
    simulation_client: &State<Box<dyn SimulationClient>>,
) -> Result<HtmxTemplate<SimulationFormTemplate>, HtmxTemplate<BannerError>> {
    let electrolyzers = electrolyzer_client
        .list_electrolyzers()
        .map_err(BannerError::create_from_error)?;
    let simulation_state = simulation_client
        .get_simulation_state(&context.simulation_id())
        .map_err(BannerError::create_from_error)?;

    Ok(SimulationFormTemplate {
        generation_range: DateTimeRange {
            start: String::from("2023-01-01T00:00"),
            end: String::from("2023-07-31T23:59"),
        },
        electrolyzer_selector: ElectrolyzerSelectorTemplate {
            electrolyzers,
            selected_id: simulation_state.electrolyzer.id,
        },
    }
    .into())
}
