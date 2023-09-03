use rocket::{form::Form, post, State};

use crate::{
    persistance::electrolyzer::ElectrolyzerPersistanceClient,
    responders::htmx_responder::{HtmxHeaders, HtmxTemplate},
    schema::electrolyzer::{
        ConstantProduction, CreateElectrolyzerRespone, CreateElectrolzyerRequest, Electrolyzer,
    },
};

#[post("/create_electrolyzer", data = "<request>")]
pub fn create_electrolyzer_handler(
    request: Form<CreateElectrolzyerRequest>,
    electrolyzer_client: &State<Box<dyn ElectrolyzerPersistanceClient>>,
) -> HtmxTemplate<CreateElectrolyzerRespone> {
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
    headers.trigger = Some(String::from("electrolyzer_created"));

    HtmxTemplate::new(CreateElectrolyzerRespone { electrolyzer }, headers)
}
