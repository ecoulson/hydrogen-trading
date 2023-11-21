use askama::Template;
use rocket::{post, State};

use crate::{
    components::component::{Component, ComponentResponse},
    logic::simulation::SimulationState,
    persistance::simulation::SimulationClient,
    schema::errors::BannerError,
};

#[derive(Debug, Template)]
#[template(path = "components/list_simulations.html")]
pub struct ListSimulationResponse {
    pub simulations: Vec<SimulationState>,
}

#[post("/list_simulations")]
pub fn list_simulation_handler(
    simulation_client: &State<Box<dyn SimulationClient>>,
) -> ComponentResponse<ListSimulationResponse, BannerError> {
    Component::htmx(ListSimulationResponse {
        simulations: simulation_client.list_simulations()?,
    })
}
