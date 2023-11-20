use rocket::{post, State};

use crate::responders::user_context::UserContext;
use crate::templates::list_electrolyzers_template::ElectrolyzerSelectorTemplate;
use crate::{
    persistance::{electrolyzer::ElectrolyzerClient, simulation::SimulationClient},
    responders::htmx_responder::HtmxTemplate,
    schema::errors::BannerError,
};

#[post("/electrolyzer_selector")]
pub fn electrolyzer_selector_handler(
    user_context: UserContext,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
    simulation_client: &State<Box<dyn SimulationClient>>,
) -> Result<HtmxTemplate<ElectrolyzerSelectorTemplate>, HtmxTemplate<BannerError>> {
    let user = user_context
        .user()
        .ok_or_else(|| BannerError::create_from_message("User not logged in"))?;
    let simulation = simulation_client
        .get_simulation_state(&user.simulation_id())
        .map_err(BannerError::create_from_error)?;

    Ok(ElectrolyzerSelectorTemplate {
        selected_id: simulation.electrolyzer_id,
        electrolyzers: electrolyzer_client
            .list_electrolyzers()
            .map_err(BannerError::create_from_error)?,
    }
    .into())
}
