use rocket::{post, State};

use crate::{
    components::component::{Component, ComponentResponse},
    persistance::{electrolyzer::ElectrolyzerClient, simulation::SimulationClient},
    schema::{errors::BannerError, user::User},
    templates::list_electrolyzers_template::ElectrolyzerSelectorTemplate,
};

#[post("/electrolyzer_selector")]
pub fn electrolyzer_selector_handler(
    user: User,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
    simulation_client: &State<Box<dyn SimulationClient>>,
) -> ComponentResponse<ElectrolyzerSelectorTemplate, BannerError> {
    let simulation = simulation_client.get_simulation_state(&user.simulation_id())?;

    Component::htmx(ElectrolyzerSelectorTemplate {
        selected_id: simulation.electrolyzer_id,
        electrolyzers: electrolyzer_client.list_electrolyzers()?,
    })
}
