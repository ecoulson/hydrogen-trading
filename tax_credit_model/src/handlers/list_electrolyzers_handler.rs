use askama::Template;
use rocket::{get, State};

use crate::{
    persistance::electrolyzer::ElectrolyzerPersistanceClient,
    responders::htmx_responder::HtmxTemplate,
    schema::{electrolyzer::Electrolyzer, errors::BannerError},
};

#[derive(Template)]
#[template(path = "components/list_electrolyzers.html")]
pub struct ListElectrolyzerTemplate {
    electrolyzers: Vec<Electrolyzer>,
}

#[get("/list_electrolyzers")]
pub fn list_electrolyzers_handler(
    electrolyzer_client: &State<Box<dyn ElectrolyzerPersistanceClient>>,
) -> Result<HtmxTemplate<ListElectrolyzerTemplate>, HtmxTemplate<BannerError>> {
    let electrolyzers = electrolyzer_client
        .list_electrolyzers()
        .map_err(|err| BannerError {
            message: err.to_string(),
        })?;

    Ok(ListElectrolyzerTemplate { electrolyzers }.into())
}
