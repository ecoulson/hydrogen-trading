use rocket::{post, State};

use crate::{
    persistance::{electrolyzer::ElectrolyzerClient, simulation::SimulationClient},
    responders::{htmx_responder::HtmxTemplate, user_context::UserContext},
    schema::{electrolyzer::ElectrolyzerDetailsTemplate, errors::BannerError},
};

#[post("/get_active_electrolyzer")]
pub fn get_active_electrolyzer_handler(
    user_context: UserContext,
    simulation_client: &State<Box<dyn SimulationClient>>,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
) -> Result<HtmxTemplate<ElectrolyzerDetailsTemplate>, HtmxTemplate<BannerError>> {
    let user = user_context
        .user()
        .ok_or_else(|| BannerError::create_from_message("User not logged in"))?;
    let simulation = simulation_client
        .get_simulation_state(&user.simulation_id())
        .map_err(BannerError::create_from_error)?;
    let electrolyzer = electrolyzer_client
        .get_electrolyzer(simulation.electrolyzer_id)
        .map_err(BannerError::create_from_error)?;

    Ok(ElectrolyzerDetailsTemplate {
        electrolyzer,
        selected: true,
    }
    .into())
}
