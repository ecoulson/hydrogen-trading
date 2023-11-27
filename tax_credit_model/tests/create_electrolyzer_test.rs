use crate::utils::temp_dir::TempDirectory;

use tax_credit_model_server::{
    persistance::{
        electrolyzer::InMemoryElectrolyzerPersistanceClient,
        generation::DiskGenerationPersistanceClient, grid::InMemoryGridClient,
        simulation::InMemorySimulationClient,
    },
    schema::electrolyzer::{
        ConstantProduction, CreateElectrolyzerRequest, ElectrolyzerDetails,
    },
    server::Dependencies,
};
use utils::{test_env::TestEnv, test_server::Method};

mod utils;

#[rocket::async_test]
async fn create_electrolyzer_successfully() {
    let directory = TempDirectory::create_from_env("TMPDIR", "create_electrolyzer").unwrap();
    let path = TempDirectory::canonicalize_path(&directory, "data.txt");
    let dependencies = Dependencies {
        electrolyzer_client: Box::new(InMemoryElectrolyzerPersistanceClient::new()),
        grid_client: Box::new(InMemoryGridClient::new()),
        simulation_client: Box::new(InMemorySimulationClient::new()),
        generation_client: Box::new(DiskGenerationPersistanceClient::new(&path).unwrap()),
    };
    let mut request = CreateElectrolyzerRequest::default();
    request.production_method.conversion_rate_constant = Some(0.5);
    let mut expected_response = ElectrolyzerDetails::default();
    expected_response.electrolyzer.constant_production = Some(ConstantProduction {
        conversion_rate: 0.5,
    });

    let server = TestEnv::load().create_test_server(dependencies).await;
    let response = server
        .invoke_template::<CreateElectrolyzerRequest>(
            Method::Post,
            "/create_electrolyzer",
            &request,
        )
        .await;

    assert_template_eq!(response.data, expected_response);
}
