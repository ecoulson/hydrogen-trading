use rocket::{form::Form, post, FromForm, State};

use crate::{
    persistance::electrolyzer::ElectrolyzerClient, responders::htmx_responder::HtmxTemplate,
    schema::errors::BannerError, templates::list_electrolyzers_template::ElectrolyzerSearchResults,
};

#[derive(FromForm)]
pub struct SearchElectrolyzersRequest {
    simulation_id: i32,
    query: String,
}

#[post("/search_electrolyzers", data = "<request>")]
pub fn search_electrolyzers_handler(
    request: Form<SearchElectrolyzersRequest>,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
) -> Result<HtmxTemplate<ElectrolyzerSearchResults>, HtmxTemplate<BannerError>> {
    if request.query.trim().is_empty() {
        return Ok(ElectrolyzerSearchResults {
            simulation_id: request.simulation_id,
            electrolyzers: electrolyzer_client
                .list_electrolyzers()
                .map_err(BannerError::create_from_error)?,
        }
        .into());
    }

    Ok(ElectrolyzerSearchResults {
        simulation_id: request.simulation_id,
        electrolyzers: electrolyzer_client
            .list_electrolyzers()
            .map_err(BannerError::create_from_error)?
            .into_iter()
            .filter(|electrolyzer| {
                electrolyzer
                    .name
                    .to_lowercase()
                    .starts_with(&request.query.to_lowercase())
            })
            .collect(),
    }
    .into())
}
