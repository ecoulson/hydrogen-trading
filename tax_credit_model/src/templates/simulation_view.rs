use askama::Template;

use crate::{
    client::events::ClientEvent,
    components::{
        button::{Button, ButtonBuilder, ButtonVariant},
        event::{EventListener, EventListenerBuilder},
    },
    schema::time::DateTimeRange,
};

use super::list_electrolyzers_template::ElectrolyzerSelectorTemplate;

#[derive(Template, Default, Debug)]
#[template(path = "components/simulation_view.html")]
pub struct SimulationView {
    generation_range: DateTimeRange,
    electrolyzer_selector: ElectrolyzerSelectorTemplate,
    create_electrolyzer_listener: EventListener,
    list_simulation_button: Button,
    simulate_button: Button,
}

pub struct SimulationViewBuilder {
    simulation_view: SimulationView,
}

impl SimulationViewBuilder {
    pub fn new() -> Self {
        Self {
            simulation_view: SimulationView {
                create_electrolyzer_listener: EventListenerBuilder::new()
                    .event(ClientEvent::CreateElectrolyzer)
                    .endpoint("/get_selected_simulation")
                    .target("#dataplane")
                    .build(),
                generation_range: DateTimeRange::default(),
                electrolyzer_selector: ElectrolyzerSelectorTemplate::default(),
                list_simulation_button: ButtonBuilder::new()
                    .endpoint("/list_simulations")
                    .target("#dataplane")
                    .text("View Runs")
                    .variant(ButtonVariant::Outline)
                    .build(),
                simulate_button: ButtonBuilder::new()
                    .endpoint("/execute_simulation")
                    .target("#simulation-result")
                    .text("Simulate")
                    .variant(ButtonVariant::Secondary)
                    .build(),
            },
        }
    }

    pub fn generation_range(mut self, generation_range: DateTimeRange) -> Self {
        self.simulation_view.generation_range = generation_range;

        self
    }

    pub fn electrolyzer_selector(
        mut self,
        electrolyzer_selector: ElectrolyzerSelectorTemplate,
    ) -> Self {
        self.simulation_view.electrolyzer_selector = electrolyzer_selector;

        self
    }

    pub fn build(self) -> SimulationView {
        self.simulation_view
    }
}
