use rocket::{form::Form, post, State};

use crate::{
    persistance::electrolyzer::ElectrolyzerClient,
    responders::htmx_responder::{HtmxHeaders, HtmxTemplate, HX_TRIGGER},
    schema::{
        electrolyzer::{
            ConstantProduction, CreateElectrolyzerRequest, CreateElectrolyzerResponse,
            Electrolyzer, ProductionType,
        },
        errors::BannerError,
    },
};

#[post("/create_electrolyzer", data = "<request>")]
pub fn create_electrolyzer_handler(
    request: Form<CreateElectrolyzerRequest>,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
) -> Result<HtmxTemplate<CreateElectrolyzerResponse>, HtmxTemplate<BannerError>> {
    let electrolyzer = electrolyzer_client
        .create_electrolyzer(&Electrolyzer {
            id: 0,
            replacement_threshold: request.replacement_threshold,
            degredation_rate: request.degredation_rate,
            capacity_mw: request.capacity_mw,
            opex: request.opex,
            capex: request.capex,
            constant_production: Some(ConstantProduction {
                conversion_rate: request
                    .production_method
                    .conversion_rate_constant
                    .ok_or_else(|| BannerError {
                        message: String::from("Only constant rate supported"),
                    })?,
            }),
            production_type: ProductionType::Constant,
            replacement_cost: request.replacement_cost,
        })
        .map_err(|err| BannerError {
            message: err.to_string(),
        })?;
    let mut headers = HtmxHeaders::default();
    headers.set_header(HX_TRIGGER, "electrolyzer_created");

    Ok(HtmxTemplate::new(
        CreateElectrolyzerResponse { electrolyzer },
        headers,
    ))
}
