use rocket::{post, State};

use crate::{
    persistance::electrolyzer::ElectrolyzerClient, responders::htmx_responder::HtmxTemplate,
    schema::errors::BannerError, templates::list_electrolyzers_template::ListElectrolyzersTemplate,
};

#[post("/list_electrolyzers")]
pub fn list_electrolyzers_handler(
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
) -> Result<HtmxTemplate<ListElectrolyzersTemplate>, HtmxTemplate<BannerError>> {
    let electrolyzers = electrolyzer_client
        .list_electrolyzers()
        .map_err(BannerError::create_from_error)?;

    Ok(ListElectrolyzersTemplate { electrolyzers }.into())
}
