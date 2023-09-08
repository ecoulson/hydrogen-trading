use tax_credit_model_server::{
    persistance::electrolyzer::ElectrolyzerPersistanceClient,
    schema::{
        electrolyzer::Electrolyzer,
        simulation_schema::{ExecuteSimulationRequest, ExecuteSimulationResponse},
        time::{TimeRange, Timestamp},
        time_series::TimeSeriesEntry,
    },
};

use crate::utils::{test_env::TestEnv, test_server::Method};

mod utils;

#[rocket::async_test]
async fn test_simulate_for_simple_model() {
    let server = TestEnv::load().create_test_server().await;
    let mut time_range = TimeRange::default();
    time_range.end.seconds = 3600;
    // let end_iso_date = Timestamp::new(time_range.end.seconds, time_range.end.nanos)
    //     .to_utc_date_time()
    //     .unwrap()
    //     .to_rfc3339();
    let mut expected_response = ExecuteSimulationResponse::default();
    expected_response.simulation_result.emissions.id = String::from("emissions");
    expected_response
        .simulation_result
        .emissions
        .time_series
        .label = String::from("Emissions");
    expected_response
        .simulation_result
        .emissions
        .time_series
        .data_points
        .push(TimeSeriesEntry {
            date: Timestamp::new(time_range.start.seconds, time_range.start.nanos)
                .to_utc_date_time()
                .unwrap()
                .to_rfc3339(),
            value: 530.703,
        });
    expected_response.simulation_result.hydrogen_productions.id = String::from("hydrogen-produced");
    expected_response
        .simulation_result
        .hydrogen_productions
        .time_series
        .label = String::from("Hydrogen Produced");
    expected_response
        .simulation_result
        .hydrogen_productions
        .time_series
        .data_points
        .push(TimeSeriesEntry {
            date: Timestamp::new(time_range.start.seconds, time_range.start.nanos)
                .to_utc_date_time()
                .unwrap()
                .to_rfc3339(),
            value: 40.0,
        });
    let client: &Box<dyn ElectrolyzerPersistanceClient> = server.get();
    let request = ExecuteSimulationRequest::new(0, time_range);
    let electrolyzer =
        Electrolyzer::constant_production(0, 0.5, 0.5, 0.02, 5.0, 20.0, 11750.0, 0.0019);
    client
        .create_electrolyzer(&electrolyzer)
        .expect("Should create electrolyzer");

    let response = server
        .invoke_template::<ExecuteSimulationRequest>(Method::Post, "/execute_simulation", &request)
        .await;

    assert_template_eq!(response.data, expected_response);
}
