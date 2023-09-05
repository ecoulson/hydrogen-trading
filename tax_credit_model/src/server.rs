use rocket::{
    fs::{relative, FileServer},
    routes, Build, Rocket,
};

use crate::{
    handlers::{
        create_electrolyzer_handler::create_electrolyzer_handler,
        execute_simulation_handler::execute_simulation,
        list_electrolyzers_handler::list_electrolyzers_handler,
        simulation_handler::simulation_handler,
    },
    logic::grid_fetcher::{GridFetcher, InMemoryGridFetcher},
    persistance::electrolyzer::{
        ElectrolyzerPersistanceClient, InMemoryElectrolyzerPersistanceClient,
    },
};

pub struct ServerConfiguration {}

impl ServerConfiguration {
    pub fn new() -> ServerConfiguration {
        ServerConfiguration {}
    }
}

pub fn init_service(configuration: ServerConfiguration) -> Rocket<Build> {
    let path =
        std::env::var("ASSETS_DIRECTORY").unwrap_or_else(|_err| relative!("assets").to_string());

    rocket::build()
        .manage(configuration)
        .manage(Box::new(InMemoryGridFetcher::new()) as Box<dyn GridFetcher>)
        .manage(Box::new(InMemoryElectrolyzerPersistanceClient::new())
            as Box<dyn ElectrolyzerPersistanceClient>)
        .mount("/assets", FileServer::from(path))
        .mount(
            "/",
            routes![
                execute_simulation,
                create_electrolyzer_handler,
                simulation_handler,
                list_electrolyzers_handler
            ],
        )
}
