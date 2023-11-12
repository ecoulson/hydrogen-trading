use askama::Template;
use serde::{Deserialize, Serialize};

use crate::schema::electrolyzer::Electrolyzer;

#[derive(Template, Deserialize, Serialize, Default, Debug, PartialEq)]
#[template(path = "components/electrolyzer_selector.html")]
pub struct ElectrolyzerSelectorTemplate {
    pub selected_id: usize,
    pub electrolyzers: Vec<Electrolyzer>,
}

#[derive(Template, Deserialize, Serialize, Default, Debug, PartialEq)]
#[template(path = "components/list_electrolyzers.html")]
pub struct ListElectrolyzersTemplate {
    pub electrolyzers: Vec<Electrolyzer>
}
