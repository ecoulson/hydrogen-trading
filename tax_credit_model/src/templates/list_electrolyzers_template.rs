use askama::Template;
use serde::{Deserialize, Serialize};

use crate::schema::electrolyzer::{Electrolyzer, ElectrolyzerId};

#[derive(Template, Deserialize, Serialize, Default, Debug, PartialEq)]
#[template(path = "components/electrolyzer_selector.html")]
pub struct ElectrolyzerSelectorTemplate {
    pub selected_id: ElectrolyzerId,
    pub electrolyzers: Vec<Electrolyzer>,
}

#[derive(Template, Deserialize, Serialize, Default, Debug, PartialEq)]
#[template(path = "components/list_electrolyzers.html")]
pub struct ListElectrolyzersTemplate {
    pub search_results: ElectrolyzerSearchResults,
}

#[derive(Template, Deserialize, Serialize, Default, Debug, PartialEq)]
#[template(path = "components/electrolyzer_search_results.html")]
pub struct ElectrolyzerSearchResults {
    pub selected_id: Option<ElectrolyzerId>,
    pub electrolyzers: Vec<Electrolyzer>,
}
