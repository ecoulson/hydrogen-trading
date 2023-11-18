use rocket::{form::Form, post, FromForm, State};

use crate::{
    persistance::{electrolyzer::ElectrolyzerClient, simulation::SimulationClient},
    responders::htmx_responder::{HtmxHeadersBuilder, HtmxTemplate},
    schema::{electrolyzer::ElectrolyzerDetailsTemplate, errors::BannerError},
};

#[derive(FromForm, Debug, Default)]
pub struct SelectElectrolyzerHandlerRequest {
    electrolyzer_id: usize,
    simulation_id: i32,
}

#[post("/select_electrolyzer", data = "<request>")]
pub fn select_electrolyzer_handler(
    request: Form<SelectElectrolyzerHandlerRequest>,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
    simulation_client: &State<Box<dyn SimulationClient>>,
) -> Result<HtmxTemplate<ElectrolyzerDetailsTemplate>, HtmxTemplate<BannerError>> {
    let mut state = simulation_client
        .get_simulation_state(&request.simulation_id)
        .map_err(BannerError::create_from_error)?;
    let electrolyzer = electrolyzer_client
        .get_electrolyzer(request.electrolyzer_id)
        .map_err(BannerError::create_from_error)?;
    state.electrolyzer_id = electrolyzer.id;
    simulation_client
        .update(&state)
        .map_err(BannerError::create_from_error)?;

    Ok(HtmxTemplate::new(
        HtmxHeadersBuilder::new()
            .trigger("electrolyzer-selected")
            .build(),
        ElectrolyzerDetailsTemplate {
            electrolyzer,
            selected: true,
            simulation_id: state.id,
        },
    ))
}
