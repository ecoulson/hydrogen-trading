use rocket::{form::Form, post, FromForm, State};

use crate::{
    persistance::{electrolyzer::ElectrolyzerClient, simulation::SimulationClient},
    responders::{
        context::Context,
        htmx_responder::{HtmxHeadersBuilder, HtmxTemplate},
    },
    schema::{electrolyzer::ElectrolyzerDetailsTemplate, errors::BannerError},
};

#[derive(FromForm, Debug, Default)]
pub struct SelectElectrolyzerHandlerRequest {
    electrolyzer_id: usize,
}

#[post("/select_electrolyzer", data = "<request>")]
pub fn select_electrolyzer_handler(
    context: Context,
    request: Form<SelectElectrolyzerHandlerRequest>,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
    simulation_client: &State<Box<dyn SimulationClient>>,
) -> Result<HtmxTemplate<ElectrolyzerDetailsTemplate>, HtmxTemplate<BannerError>> {
    let mut state = simulation_client
        .get_simulation_state(&context.simulation_id())
        .map_err(BannerError::create_from_error)?;
    state.electrolyzer = electrolyzer_client
        .get_electrolyzer(request.electrolyzer_id)
        .map_err(BannerError::create_from_error)?;
    simulation_client
        .update(&state)
        .map_err(BannerError::create_from_error)?;

    Ok(HtmxTemplate::new(
        HtmxHeadersBuilder::new()
            .trigger("electrolyzer-selected")
            .build(),
        ElectrolyzerDetailsTemplate {
            electrolyzer: state.electrolyzer,
            selected: true,
        },
    ))
}
