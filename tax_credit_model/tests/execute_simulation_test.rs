use tax_credit_model_server::{
    persistance::{
        electrolyzer::InMemoryElectrolyzerPersistanceClient, grid::InMemoryGridClient,
        simulation::InMemorySimulationClient,
    },
    schema::{
        electrolyzer::{ConstantProduction, Electrolyzer},
        simulation_schema::{
            EnergySourcePortfolio, ExecuteSimulationRequest, ExecuteSimulationResponse,
            GenerationMetric,
        },
        time::{DateTimeRange, TimeRange, Timestamp},
    },
    server::Dependencies,
};

use crate::utils::{test_env::TestEnv, test_server::Method};

mod utils;

#[rocket::async_test]
async fn test_simulate_for_simple_model() {
    let dependencies = Dependencies {
        electrolyzer_client: Box::new(InMemoryElectrolyzerPersistanceClient::new()),
        grid_client: Box::new(InMemoryGridClient::new()),
        simulation_client: Box::new(InMemorySimulationClient::new()),
    };
    let mut time_range = TimeRange::default();
    time_range.end.seconds = 3600;
    let mut expected_response = ExecuteSimulationResponse::default();
    expected_response
        .simulation_result
        .tax_credit_summary
        .credit_hours_none = 0.25;
    expected_response.simulation_result.emissions.id = String::from("emissions");
    expected_response.simulation_result.emissions.title = String::from("Emissions Over Time");
    expected_response
        .simulation_result
        .emissions
        .data_set_endpoints
        .push(String::from("fetch_emissions/0"));
    expected_response.simulation_result.hydrogen_productions.id = String::from("hydrogen-produced");
    expected_response
        .simulation_result
        .hydrogen_productions
        .title = String::from("Hydrogen Production Over Time");
    expected_response
        .simulation_result
        .hydrogen_productions
        .data_set_endpoints
        .push(String::from("fetch_hydrogen_production/0"));
    expected_response.simulation_result.energy_costs.id = String::from("energy-costs");
    expected_response.simulation_result.energy_costs.title = String::from("Energy Costs Over Time");
    expected_response
        .simulation_result
        .energy_costs
        .data_set_endpoints
        .push(String::from("fetch_energy_costs/0"));
    let mut electrolyzer = Electrolyzer::default();
    electrolyzer.capacity_mw = 100.0;
    electrolyzer.constant_production = Some(ConstantProduction {
        conversion_rate: 20.0,
    });
    dependencies
        .electrolyzer_client
        .create_electrolyzer(&electrolyzer)
        .unwrap();
    let mut start = 0;
    let mut generations = vec![];
    // Find a better way to hard code this
    while start < time_range.end.seconds {
        let mut portfolio = EnergySourcePortfolio::default();
        portfolio.natural_gas_mwh = 2.0;
        portfolio.total_electricity_mwh = 2.0;
        let generation = GenerationMetric::new(0, &Timestamp::new(start, 0), 2.0, portfolio);
        generations.push(generation);
        start += 15 * 60; // 15 minutes in seconds
    }
    dependencies
        .grid_client
        .add_generations(generations)
        .unwrap();
    let request = ExecuteSimulationRequest::new(
        0,
        DateTimeRange {
            start: String::from("1970-01-01T00:00"),
            end: String::from("1970-01-01T00:15"),
        },
    );

    let server = TestEnv::load().create_test_server(dependencies).await;
    let response = server
        .invoke_template::<ExecuteSimulationRequest>(Method::Post, "/execute_simulation", &request)
        .await;

    assert_template_eq!(response.data, expected_response);
}
