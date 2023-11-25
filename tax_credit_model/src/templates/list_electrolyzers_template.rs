use askama::Template;
use serde::{Deserialize, Serialize};

use crate::{
    components::{
        button::{Button, ButtonBuilder, ButtonVariant},
        event::EventListener,
    },
    schema::electrolyzer::{Electrolyzer, ElectrolyzerId},
};

#[derive(Template, Deserialize, Serialize, Default, Debug, PartialEq)]
#[template(path = "components/electrolyzer_selector.html")]
pub struct ElectrolyzerSelectorTemplate {
    pub selected_id: ElectrolyzerId,
    pub electrolyzers: Vec<Electrolyzer>,
    pub select_electrolyzer_listener: EventListener,
}

#[derive(Template, Deserialize, Serialize, Default, Debug, PartialEq)]
#[template(path = "components/list_electrolyzers.html")]
pub struct ListElectrolyzersTemplate {
    pub search_results: ElectrolyzerSearchResults,
    pub list_simulation_listener: EventListener,
    pub select_simulation_listener: EventListener,
    pub create_electrolyzer_button: Button,
}

#[derive(Template, Deserialize, Serialize, Default, Debug, PartialEq)]
#[template(path = "components/electrolyzer_search_results.html")]
pub struct ElectrolyzerSearchResults {
    pub selected_id: Option<ElectrolyzerId>,
    pub electrolyzers: Vec<Electrolyzer>,
    pub select_electrolyzer_button: Button,
}

pub struct ElectrolyzerSearchResultsBuilder {
    electrolyzer_search_results: ElectrolyzerSearchResults,
}

impl ElectrolyzerSearchResultsBuilder {
    pub fn new() -> Self {
        Self {
            electrolyzer_search_results: ElectrolyzerSearchResults {
                selected_id: None,
                electrolyzers: vec![],
                select_electrolyzer_button: ButtonBuilder::new()
                    .text("Use")
                    .endpoint("/select_electrolyzer")
                    .variant(ButtonVariant::Outline)
                    .build(),
            },
        }
    }

    pub fn selected_id(mut self, selected_id: ElectrolyzerId) -> Self {
        self.electrolyzer_search_results.selected_id = Some(selected_id);

        self
    }

    pub fn electrolyzers(mut self, electrolyzers: Vec<Electrolyzer>) -> Self {
        self.electrolyzer_search_results.electrolyzers = electrolyzers;

        self
    }

    pub fn build(self) -> ElectrolyzerSearchResults {
        self.electrolyzer_search_results
    }
}
