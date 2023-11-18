use rocket::{form::Form, post, FromForm, State};

use crate::templates::list_electrolyzers_template::ElectrolyzerSelectorTemplate;
use crate::{
    persistance::{electrolyzer::ElectrolyzerClient, simulation::SimulationClient},
    responders::htmx_responder::HtmxTemplate,
    schema::errors::BannerError,
};

#[derive(FromForm)]
pub struct ElectrolyzerSelectorRequest {
    simulation_id: i32,
}

#[post("/electrolyzer_selector", data = "<request>")]
pub fn electrolyzer_selector_handler(
    request: Form<ElectrolyzerSelectorRequest>,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
    simulation_client: &State<Box<dyn SimulationClient>>,
) -> Result<HtmxTemplate<ElectrolyzerSelectorTemplate>, HtmxTemplate<BannerError>> {
    let simulation = simulation_client
        .get_simulation_state(&request.simulation_id)
        .map_err(BannerError::create_from_error)?;

    Ok(ElectrolyzerSelectorTemplate {
        selected_id: simulation.electrolyzer_id,
        simulation_id: request.simulation_id,
        electrolyzers: electrolyzer_client
            .list_electrolyzers()
            .map_err(BannerError::create_from_error)?,
    }
    .into())
}
