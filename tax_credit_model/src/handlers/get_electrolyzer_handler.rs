use rocket::{get, State};

use crate::{
    persistance::electrolyzer::ElectrolyzerClient,
    responders::htmx_responder::HtmxTemplate,
    schema::{electrolyzer::ElectrolyzerDetailsTemplate, errors::BannerError},
};

#[get("/get_electrolyzer?<electrolyzer_id>")]
pub fn get_electrolyzer_handler(
    electrolyzer_id: usize,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
) -> Result<HtmxTemplate<ElectrolyzerDetailsTemplate>, HtmxTemplate<BannerError>> {
    Ok(electrolyzer_client
        .get_electrolyzer(electrolyzer_id)
        .map_err(|err| BannerError {
            message: err.to_string(),
        })
        .map(|electrolyzer| ElectrolyzerDetailsTemplate { electrolyzer })?
        .into())
}
