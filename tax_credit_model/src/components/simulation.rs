use askama::Template;

use crate::{
    client::{events::ClientEvent, htmx::HtmxSwap},
    logic::simulation::SimulationState,
    schema::{simulation_schema::SimulationId, time::DateTimeRange},
};

use super::{
    button::Button, electrolyzer::ElectrolyzerSelector, event::EventListener, input::Input,
};

#[derive(Debug, Default, Template)]
#[template(path = "components/list_simulations.html")]
pub struct SimulationList {
    pub simulations: Vec<SimulationListItem>,
    pub create_electrolyzer_listener: EventListener,
}

impl SimulationList {
    pub fn render(simulations: Vec<SimulationState>) -> SimulationList {
        let list_items = simulations.iter().map(|simulation| SimulationListItem {
            id: simulation.id,
            id_input: Input::render_hidden(&simulation.id.to_string(), "simulation_id"),
        });

        SimulationList {
            simulations: list_items.collect(),
            create_electrolyzer_listener: EventListener::render(
                ClientEvent::CreateElectrolyzer,
                "/initialize_simulation",
                "#dataplane",
                HtmxSwap::default(),
            ),
        }
    }
}

#[derive(Template, Debug, Default)]
#[template(path = "components/simulation_list_item.html")]
pub struct SimulationListItem {
    id: SimulationId,
    id_input: Input,
}

#[derive(Template, Default, Debug)]
#[template(path = "components/simulation_view.html")]
pub struct SimulationView {
    generation_range: DateTimeRange,
    electrolyzer_selector: ElectrolyzerSelector,
    create_electrolyzer_listener: EventListener,
    list_simulation_button: Button,
    simulate_button: Button,
}

impl SimulationView {
    pub fn render(
        generation_range: DateTimeRange,
        electrolyzer_selector: ElectrolyzerSelector,
    ) -> Self {
        SimulationView {
            create_electrolyzer_listener: EventListener::render(
                ClientEvent::CreateElectrolyzer,
                "/get_selected_simulation",
                "#dataplane",
                HtmxSwap::default(),
            ),
            generation_range,
            electrolyzer_selector,
            list_simulation_button: Button::render_outline(
                "View Runs",
                "/list_simulations",
                "#dataplane",
            ),
            simulate_button: Button::render_secondary(
                "Simulate",
                "/execute_simulation",
                "#simulation-result",
            ),
        }
    }
}
