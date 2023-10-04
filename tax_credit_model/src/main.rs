use rocket::fs::relative;
use tax_credit_model_server::{
    data_retriever::fill_generations,
    persistance::{electrolyzer::InMemoryElectrolyzerPersistanceClient, grid::InMemoryGridFetcher},
    server::{init_service, Dependencies, ServerConfiguration},
};

#[macro_use]
extern crate rocket;

#[launch]
pub async fn rocket() -> _ {
    let assets_directory =
        std::env::var("ASSETS_DIRECTORY").unwrap_or_else(|_| relative!("assets").to_string());
    let data_directory =
        std::env::var("DATA_DIRECTORY").unwrap_or_else(|_| relative!("../data").to_string());
    let configuration = ServerConfiguration::new(&data_directory, &assets_directory);
    let dependencies = Dependencies {
        grid_client: Box::new(InMemoryGridFetcher::new()),
        electrolyzer_client: Box::new(InMemoryElectrolyzerPersistanceClient::new()),
    };

    fill_generations(configuration.clone(), &dependencies);
    let server = init_service(configuration.clone(), dependencies);

    server
}
