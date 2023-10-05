use askama::Template;
use serde::{Deserialize, Serialize};

use crate::schema::electrolyzer::Electrolyzer;

#[derive(Template, Deserialize, Serialize, Default, Debug, PartialEq)]
#[template(path = "components/electrolyzer_selector.html")]
pub struct ElectrolyzerSelectorTemplate {
    pub electrolyzers: Vec<Electrolyzer>,
}
