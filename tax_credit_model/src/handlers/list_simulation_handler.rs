use askama::Template;
use rocket::{post, State};

use crate::{
    logic::simulation::SimulationState, persistance::simulation::SimulationClient,
    responders::htmx_responder::HtmxTemplate, schema::errors::BannerError,
};

#[derive(Debug, Template)]
#[template(path = "components/list_simulations.html")]
pub struct ListSimulationResponse {
    simulations: Vec<SimulationState>,
}

#[post("/list_simulations")]
pub fn list_simulation_handler(
    simulation_client: &State<Box<dyn SimulationClient>>,
) -> Result<HtmxTemplate<ListSimulationResponse>, HtmxTemplate<BannerError>> {
    let simulations = simulation_client
        .list_simulations()
        .map_err(BannerError::create_from_error)?;

    Ok(ListSimulationResponse { simulations }.into())
}
