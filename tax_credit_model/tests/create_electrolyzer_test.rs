use tax_credit_model_server::{
    responders::htmx_responder::HX_TRIGGER,
    schema::electrolyzer::{
        ConstantProduction, CreateElectrolyzerRequest, CreateElectrolyzerResponse,
    },
};
use utils::{test_env::TestEnv, test_server::Method};

mod utils;

#[rocket::async_test]
async fn create_electrolyzer_successfully() {
    let server = TestEnv::load().create_test_server().await;
    let mut request = CreateElectrolyzerRequest::default();
    request.production_method.conversion_rate_constant = Some(0.5);
    let mut expected_response = CreateElectrolyzerResponse::default();
    expected_response.electrolyzer.constant_production = Some(ConstantProduction {
        conversion_rate: 0.5,
    });

    let response = server
        .invoke_template::<CreateElectrolyzerRequest>(
            Method::Post,
            "/create_electrolyzer",
            &request,
        )
        .await;

    assert_eq!(
        response.headers.get(HX_TRIGGER),
        Some(&String::from("electrolyzer_created"))
    );
    assert_template_eq!(response.data, expected_response);
}
