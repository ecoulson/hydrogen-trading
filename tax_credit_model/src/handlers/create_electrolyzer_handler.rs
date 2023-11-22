use rocket::{form::Form, post, State};

use crate::{
    components::component::{Component, ComponentResponse},
    persistance::electrolyzer::ElectrolyzerClient,
    responders::htmx_responder::HtmxHeadersBuilder,
    schema::{
        electrolyzer::{
            ConstantProduction, CreateElectrolyzerRequest, Electrolyzer,
            ElectrolyzerDetailsTemplate, ProductionType,
        },
        errors::BannerError,
    },
};

#[post("/create_electrolyzer", data = "<request>")]
pub fn create_electrolyzer_handler(
    request: Form<CreateElectrolyzerRequest>,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
) -> ComponentResponse<ElectrolyzerDetailsTemplate, BannerError> {
    let electrolyzers = electrolyzer_client.list_electrolyzers()?;
    let electrolyzer = electrolyzer_client.create_electrolyzer(&Electrolyzer {
        id: 0,
        name: String::from(&request.name),
        replacement_threshold: request.replacement_threshold,
        degredation_rate: request.degredation_rate,
        capacity_mw: request.capacity_mw,
        opex: request.opex,
        capex: request.capex,
        production: ConstantProduction {
            conversion_rate: request
                .production_method
                .conversion_rate_constant
                .ok_or_else(|| {
                    BannerError::create_from_message("Only constant production is allowed")
                })?,
        },
        production_type: ProductionType::Constant,
        replacement_cost: request.replacement_cost,
        city: String::from("Huston"),
        state: String::from("TX"),
    })?;

    Component::component(
        HtmxHeadersBuilder::new()
            .trigger("create-electrolyzer")
            .build(),
        ElectrolyzerDetailsTemplate {
            electrolyzer,
            selected: electrolyzers.is_empty(),
            selectable: true,
        },
    )
}
