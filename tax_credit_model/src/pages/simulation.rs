use askama::Template;

use crate::components::{electrolyzer::ElectrolyzerDetails, simulation::SimulationView};

#[derive(Template, Default, Debug)]
#[template(path = "pages/simulation.html")]
pub struct SimulationPage {
    pub simulation_view: SimulationView,
    pub electrolyzer_details: ElectrolyzerDetails,
}
