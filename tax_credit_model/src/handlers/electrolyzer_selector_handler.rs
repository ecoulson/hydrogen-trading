use rocket::{post, State};

use crate::{
    persistance::{electrolyzer::ElectrolyzerClient, simulation::SimulationClient},
    responders::{context::Context, htmx_responder::HtmxTemplate},
    schema::errors::BannerError,
    templates::list_electrolyzers_template::ElectrolyzerSelectorTemplate,
};

#[post("/electrolyzer_selector")]
pub fn electrolyzer_selector_handler(
    context: Context,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
    simulation_client: &State<Box<dyn SimulationClient>>,
) -> Result<HtmxTemplate<ElectrolyzerSelectorTemplate>, HtmxTemplate<BannerError>> {
    let electrolyzers = electrolyzer_client
        .list_electrolyzers()
        .map_err(BannerError::create_from_error)?;
    let simulation = simulation_client
        .get_simulation_state(&context.simulation_id())
        .map_err(BannerError::create_from_error)?;

    Ok(ElectrolyzerSelectorTemplate {
        selected_id: simulation.electrolyzer.id,
        electrolyzers,
    }
    .into())
}
