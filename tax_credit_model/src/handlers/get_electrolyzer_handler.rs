use rocket::{form::Form, post, FromForm, State};

use crate::{
    persistance::electrolyzer::ElectrolyzerClient,
    responders::htmx_responder::HtmxTemplate,
    schema::{electrolyzer::ElectrolyzerDetailsTemplate, errors::BannerError},
};

#[derive(FromForm)]
pub struct GetElectrolyzerRequest {
    pub electrolyzer_id: usize,
}

#[post("/get_electrolyzer", data = "<request>")]
pub fn get_electrolyzer_handler(
    request: Form<GetElectrolyzerRequest>,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
) -> Result<HtmxTemplate<ElectrolyzerDetailsTemplate>, HtmxTemplate<BannerError>> {
    Ok(electrolyzer_client
        .get_electrolyzer(request.electrolyzer_id)
        .map_err(BannerError::create_from_error)
        .map(|electrolyzer| ElectrolyzerDetailsTemplate { electrolyzer })?
        .into())
}
