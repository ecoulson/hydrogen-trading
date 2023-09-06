use tax_credit_model_server::schema::electrolyzer::{
    CreateElectrolyzerRequest, CreateElectrolyzerResponse,
};
use utils::test_server::Method::Post;

use crate::utils::test_env::TestEnv;

mod utils;

#[rocket::async_test]
async fn create_electrolyzer_successfully() {
    let server = TestEnv::load().create_test_server().await;
    let mut request = CreateElectrolyzerRequest::default();
    request.production_method.conversion_rate_constant = Some(0.5);
    let expected_response = CreateElectrolyzerResponse::default();

    let response = server
        .invoke_template::<CreateElectrolyzerRequest>(Post, "/create_electrolyzer", &request)
        .await;
    dbg!(&response);
    dbg!(expected_response.to_string());

    assert_template_eq!(response, expected_response);
}
