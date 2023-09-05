use askama::Template;
use rocket::{get, State};

use crate::{
    persistance::electrolyzer::ElectrolyzerPersistanceClient,
    responders::htmx_responder::HtmxTemplate, schema::electrolyzer::Electrolyzer,
};

#[derive(Template)]
#[template(path = "list_electrolyzers_template.html")]
pub struct ListElectrolyzerTemplate {
    electrolyzers: Vec<Electrolyzer>,
}

#[get("/list_electrolyzers")]
pub fn list_electrolyzers_handler(
    electrolyzer_client: &State<Box<dyn ElectrolyzerPersistanceClient>>,
) -> HtmxTemplate<ListElectrolyzerTemplate> {
    let electrolyzers = electrolyzer_client
        .list_electrolyzers()
        .expect("Should list electrolyzers");

    ListElectrolyzerTemplate { electrolyzers }.into()
}
