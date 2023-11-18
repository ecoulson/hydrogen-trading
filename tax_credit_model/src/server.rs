use rocket::{fs::FileServer, routes, Build, Rocket};

use crate::{
    handlers::{
        close_error_handler::close_error_handler,
        create_electrolyzer_form_handler::create_electrolyzer_form_handler,
        create_electrolyzer_handler::create_electrolyzer_handler,
        electrolyzer_selector_handler::electrolyzer_selector_handler,
        execute_simulation_handler::execute_simulation,
        fetch_emissions_handler::fetch_emissions_handler,
        fetch_energy_costs_handler::fetch_energy_costs_handler,
        fetch_hydrogen_production_handler::fetch_hydrogen_production_handler,
        get_electrolyzer_handler::get_electrolyzer_handler,
        get_simulation_form_handler::get_simulation_form_handler,
        list_electrolyzers_handler::list_electrolyzers_handler,
        select_electrolyzer_handler::select_electrolyzer_handler,
        simulation_handler::simulation_handler, search_electrolyzers_handler::search_electrolyzers_handler, list_simulation_handler::list_simulation_handler,
    },
    persistance::{
        electrolyzer::ElectrolyzerClient, generation::GenerationClient, grid::GridClient,
        simulation::SimulationClient,
    },
};

#[derive(Debug, Clone)]
pub struct ServingConfiguration {
    pub assets_directory: String,
}

#[derive(Debug, Clone)]
pub struct ServerConfiguration {
    pub data_directory: String,
    pub serving: ServingConfiguration,
}

impl ServerConfiguration {
    pub fn new(data_directory: &str, assets_directory: &str) -> ServerConfiguration {
        ServerConfiguration {
            data_directory: String::from(data_directory),
            serving: ServingConfiguration {
                assets_directory: String::from(assets_directory),
            },
        }
    }
}

pub struct Dependencies {
    pub electrolyzer_client: Box<dyn ElectrolyzerClient>,
    pub grid_client: Box<dyn GridClient>,
    pub simulation_client: Box<dyn SimulationClient>,
    pub generation_client: Box<dyn GenerationClient>,
}

pub fn init_service(
    configuration: ServerConfiguration,
    dependencies: Dependencies,
) -> Rocket<Build> {
    let static_files = FileServer::from(&configuration.serving.assets_directory);

    rocket::build()
        .manage(configuration)
        .manage(dependencies.grid_client)
        .manage(dependencies.electrolyzer_client)
        .manage(dependencies.simulation_client)
        .manage(dependencies.generation_client)
        .mount("/assets", static_files)
        .mount(
            "/",
            routes![
                execute_simulation,
                create_electrolyzer_handler,
                simulation_handler,
                get_simulation_form_handler,
                create_electrolyzer_form_handler,
                get_electrolyzer_handler,
                fetch_emissions_handler,
                fetch_hydrogen_production_handler,
                fetch_energy_costs_handler,
                close_error_handler,
                list_electrolyzers_handler,
                select_electrolyzer_handler,
                electrolyzer_selector_handler,
                search_electrolyzers_handler,
                list_simulation_handler
            ],
        )
}
