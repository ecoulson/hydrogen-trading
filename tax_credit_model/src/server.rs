use rocket::{fs::FileServer, routes, Build, Rocket};

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

pub struct ServingConfiguration {
    pub assets_directory: String,
}

pub struct ServerConfiguration {
    pub serving: ServingConfiguration,
}

impl ServerConfiguration {
    pub fn new(assets_directory: &str) -> ServerConfiguration {
        ServerConfiguration {
            serving: ServingConfiguration {
                assets_directory: String::from(assets_directory),
            },
        }
    }
}

pub fn init_service(configuration: ServerConfiguration) -> Rocket<Build> {
    let static_files = FileServer::from(&configuration.serving.assets_directory);

    rocket::build()
        .manage(configuration)
        .manage(Box::new(InMemoryGridFetcher::new()) as Box<dyn GridFetcher>)
        .manage(Box::new(InMemoryElectrolyzerPersistanceClient::new())
            as Box<dyn ElectrolyzerPersistanceClient>)
        .mount("/assets", static_files)
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
