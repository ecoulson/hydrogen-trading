use rocket::{get, State};

use crate::{
    persistance::electrolyzer::ElectrolyzerClient, responders::htmx_responder::HtmxTemplate,
    schema::errors::BannerError,
    templates::list_electrolyzers_template::ElectrolyzerSelectorTemplate,
};

#[get("/list_electrolyzers")]
pub fn list_electrolyzers_handler(
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
) -> Result<HtmxTemplate<ElectrolyzerSelectorTemplate>, HtmxTemplate<BannerError>> {
    let electrolyzers = electrolyzer_client
        .list_electrolyzers()
        .map_err(|err| BannerError {
            message: err.to_string(),
        })?;

    Ok(ElectrolyzerSelectorTemplate { electrolyzers }.into())
}
