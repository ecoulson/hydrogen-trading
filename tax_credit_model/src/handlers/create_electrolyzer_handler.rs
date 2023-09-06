use rocket::{form::Form, post, State};

use crate::{
    persistance::electrolyzer::ElectrolyzerPersistanceClient,
    responders::htmx_responder::{HtmxTemplate, HtmxHeaders, HX_TRIGGER},
    schema::electrolyzer::{
        ConstantProduction, CreateElectrolyzerResponse, CreateElectrolyzerRequest, Electrolyzer,
    },
};

#[post("/create_electrolyzer", data = "<request>")]
pub fn create_electrolyzer_handler(
    request: Form<CreateElectrolyzerRequest>,
    electrolyzer_client: &State<Box<dyn ElectrolyzerPersistanceClient>>,
) -> HtmxTemplate<CreateElectrolyzerResponse> {
    let electrolyzer = electrolyzer_client
        .create_electrolyzer(&Electrolyzer {
            id: 0,
            replacement_threshold: request.replacement_threshold,
            degredation_rate: request.degredation_rate,
            capacity_mw: request.capacity_mw,
            opex: request.opex,
            capex: request.capex,
            production_method: ConstantProduction {
                conversion_rate: request
                    .production_method
                    .conversion_rate_constant
                    .expect("Expected constant rate"),
            },
            replacement_cost: request.replacement_cost,
        })
        .expect("Should create electrolyzer");
    let mut headers = HtmxHeaders::default();
    headers.set_header(HX_TRIGGER, "electrolyzer_created");

    HtmxTemplate::new(CreateElectrolyzerResponse { electrolyzer }, headers)
}
