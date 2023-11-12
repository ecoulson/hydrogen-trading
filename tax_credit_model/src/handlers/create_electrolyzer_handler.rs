use rocket::{form::Form, post, State};

use crate::{
    persistance::electrolyzer::ElectrolyzerClient,
    responders::htmx_responder::{HtmxHeadersBuilder, HtmxTemplate},
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
) -> Result<HtmxTemplate<ElectrolyzerDetailsTemplate>, HtmxTemplate<BannerError>> {
    let electrolyzers = electrolyzer_client.list_electrolyzers()
        .map_err(BannerError::create_from_error)?;
    let electrolyzer = electrolyzer_client
        .create_electrolyzer(&Electrolyzer {
            id: 0,
            name: String::from(&request.name),
            replacement_threshold: request.replacement_threshold,
            degredation_rate: request.degredation_rate,
            capacity_mw: request.capacity_mw,
            opex: request.opex,
            capex: request.capex,
            constant_production: Some(ConstantProduction {
                conversion_rate: request
                    .production_method
                    .conversion_rate_constant
                    .ok_or_else(|| {
                        BannerError::create_from_message("Only constant production is allowed")
                    })?,
            }),
            production_type: ProductionType::Constant,
            replacement_cost: request.replacement_cost,
            city: String::from("Huston"),
            state: String::from("TX"),
        })
        .map_err(BannerError::create_from_error)?;

    Ok(HtmxTemplate::new(
        HtmxHeadersBuilder::new()
            .trigger("create-electrolyzer")
            .build(),
        ElectrolyzerDetailsTemplate {
            electrolyzer,
            selected: electrolyzers.is_empty(),
        },
    ))
}
