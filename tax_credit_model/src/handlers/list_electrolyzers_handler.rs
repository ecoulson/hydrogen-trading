use rocket::{form::Form, post, FromForm, State};

use crate::{
    persistance::electrolyzer::ElectrolyzerClient,
    responders::htmx_responder::HtmxTemplate,
    schema::errors::BannerError,
    templates::list_electrolyzers_template::{
        ElectrolyzerSearchResults, ListElectrolyzersTemplate,
    },
};

#[derive(Debug, FromForm)]
pub struct ListElectrolyzerRequest {
    simulation_id: i32,
}

#[post("/list_electrolyzers", data = "<request>")]
pub fn list_electrolyzers_handler(
    request: Form<ListElectrolyzerRequest>,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
) -> Result<HtmxTemplate<ListElectrolyzersTemplate>, HtmxTemplate<BannerError>> {
    let electrolyzers = electrolyzer_client
        .list_electrolyzers()
        .map_err(BannerError::create_from_error)?;

    Ok(ListElectrolyzersTemplate {
        simulation_id: request.simulation_id,
        search_results: ElectrolyzerSearchResults {
            simulation_id: request.simulation_id,
            electrolyzers,
        },
    }
    .into())
}
