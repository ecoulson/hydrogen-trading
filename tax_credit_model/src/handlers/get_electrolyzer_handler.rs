use rocket::{form::Form, post, FromForm, State};

use crate::{
    components::component::{Component, ComponentResponse},
    persistance::{electrolyzer::ElectrolyzerClient, simulation::SimulationClient},
    schema::{electrolyzer::ElectrolyzerDetailsTemplate, errors::BannerError, user::User},
};

#[derive(FromForm)]
pub struct GetElectrolyzerRequest {
    pub electrolyzer_id: usize,
}

#[post("/get_electrolyzer", data = "<request>")]
pub fn get_electrolyzer_handler(
    request: Form<GetElectrolyzerRequest>,
    user: User,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
    simulation_client: &State<Box<dyn SimulationClient>>,
) -> ComponentResponse<ElectrolyzerDetailsTemplate, BannerError> {
    let simulation = simulation_client.get_simulation_state(&user.simulation_id())?;

    electrolyzer_client
        .get_electrolyzer(request.electrolyzer_id)
        .map_err(BannerError::create_from_error)
        .map(|electrolyzer| {
            Component::basic(ElectrolyzerDetailsTemplate {
                electrolyzer,
                selected: simulation.electrolyzer_id == request.electrolyzer_id,
            })
        })?
}
