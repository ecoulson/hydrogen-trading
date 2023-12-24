use askama::Template;

use crate::{
    client::{events::ClientEvent, htmx::HtmxSwap},
    logic::simulation::SimulationState,
    schema::{
        endpoints::Endpoint,
        simulation::{SimulationId, SimulationResult},
        time::DateTimeRange,
    },
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
            endpoint: Endpoint::SelectSimulation,
        });

        SimulationList {
            simulations: list_items.collect(),
            create_electrolyzer_listener: EventListener::render(
                ClientEvent::CreateElectrolyzer,
                Endpoint::InitializeSimulation,
                "#dataplane",
                HtmxSwap::default(),
            ),
        }
    }
}

#[derive(Template, Debug, Default)]
#[template(path = "components/simulation_list_item.html")]
pub struct SimulationListItem {
    endpoint: Endpoint,
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
                Endpoint::GetSelectedSimulation,
                "#dataplane",
                HtmxSwap::default(),
            ),
            generation_range,
            electrolyzer_selector,
            list_simulation_button: Button::render_outline(
                "View Runs",
                Endpoint::ListSimulations,
                "#dataplane",
            ),
            simulate_button: Button::render_secondary(
                "Simulate",
                Endpoint::ExecuteSimulation,
                "#simulation-result",
            ),
        }
    }
}

#[derive(Template, Default, Debug)]
#[template(path = "components/simulation_result.html")]
pub struct SimulationResultView {
    pub simulation_result: SimulationResult,
    pub simulation_view: SimulationView,
}

impl SimulationResultView {
    pub fn render(simulation_view: SimulationView, simulation_result: SimulationResult) -> Self {
        Self {
            simulation_view,
            simulation_result,
        }
    }
}
