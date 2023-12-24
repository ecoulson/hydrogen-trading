use askama::Template;

use crate::components::{electrolyzer::ElectrolyzerList, simulation::SimulationList};

#[derive(Template, Debug)]
#[template(path = "pages/index.html")]
pub struct IndexResponse {
    pub electrolyzer_list: ElectrolyzerList,
    pub simulation_list: SimulationList,
}
