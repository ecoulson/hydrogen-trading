use tax_credit_model_server::schema::electrolyzer::{
    CreateElectrolyzerResponse, CreateElectrolzyerRequest,
};
use utils::test_server::Method::Post;

use crate::utils::test_env::TestEnv;

mod utils;

#[rocket::async_test]
async fn execute_simulation_successfully() {
    let server = TestEnv::load().create_test_server().await;
    let request = CreateElectrolzyerRequest::default();

    let response = server.invoke::<CreateElectrolzyerRequest, CreateElectrolyzerResponse>(
        Post,
        "/create_electrolyzer",
        &request,
    ).await;

    dbg!(&response);

    assert!(1 == 2);
}
