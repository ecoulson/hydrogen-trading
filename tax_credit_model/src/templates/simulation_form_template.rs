use askama::Template;
use serde::{Deserialize, Serialize};

use crate::schema::time::DateTimeRange;

use super::list_electrolyzers_template::ElectrolyzerSelectorTemplate;

#[derive(Template, Deserialize, Serialize, Default, Debug, PartialEq)]
#[template(path = "components/simulation_form.html")]
pub struct SimulationFormTemplate {
    pub simulation_id: i32,
    pub generation_range: DateTimeRange,
    pub electrolyzer_selector: ElectrolyzerSelectorTemplate,
}
