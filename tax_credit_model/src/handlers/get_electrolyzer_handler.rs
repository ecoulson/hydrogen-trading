use rocket::{form::Form, post, FromForm, State};

use crate::{
    persistance::{electrolyzer::ElectrolyzerClient, simulation::SimulationClient},
    responders::{context::Context, htmx_responder::HtmxTemplate},
    schema::{electrolyzer::ElectrolyzerDetailsTemplate, errors::BannerError},
};

#[derive(FromForm)]
pub struct GetElectrolyzerRequest {
    pub electrolyzer_id: usize,
}

#[post("/get_electrolyzer", data = "<request>")]
pub fn get_electrolyzer_handler(
    context: Context,
    request: Form<GetElectrolyzerRequest>,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
    simulation_client: &State<Box<dyn SimulationClient>>,
) -> Result<HtmxTemplate<ElectrolyzerDetailsTemplate>, HtmxTemplate<BannerError>> {
    let simulation = simulation_client
        .get_simulation_state(&context.simulation_id())
        .map_err(BannerError::create_from_error)?;

    Ok(electrolyzer_client
        .get_electrolyzer(request.electrolyzer_id)
        .map_err(BannerError::create_from_error)
        .map(|electrolyzer| ElectrolyzerDetailsTemplate {
            electrolyzer,
            selected: simulation.electrolyzer.id == request.electrolyzer_id,
        })?
        .into())
}
