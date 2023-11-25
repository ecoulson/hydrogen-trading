use rocket::{form::Form, post, State};

use crate::{
    client::{events::ClientEvent, htmx::HtmxSwap},
    components::{
        component::{Component, ComponentResponse},
        event::EventListenerBuilder,
    },
    logic::simulation::{simulate, SimulationState},
    persistance::{
        electrolyzer::ElectrolyzerClient, grid::GridClient, simulation::SimulationClient,
        simulation_selection::SimulationSelectionClient,
    },
    responders::{client_context::ClientContext, htmx_responder::HtmxHeadersBuilder},
    schema::{
        errors::BannerError,
        simulation_schema::{ExecuteSimulationRequest, ExecuteSimulationResponse},
        time::DateTimeRange,
        user::User,
    },
    templates::{
        list_electrolyzers_template::ElectrolyzerSelectorTemplate,
        simulation_view::SimulationViewBuilder,
    },
};

#[post("/execute_simulation", data = "<request>")]
pub fn execute_simulation(
    client_context: ClientContext,
    user: User,
    request: Form<ExecuteSimulationRequest>,
    power_grid_fetcher: &State<Box<dyn GridClient>>,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
    simulation_client: &State<Box<dyn SimulationClient>>,
    simulation_selection_client: &State<Box<dyn SimulationSelectionClient>>,
) -> ComponentResponse<ExecuteSimulationResponse, BannerError> {
    let mut client_context = client_context;
    let electrolyzer = electrolyzer_client.get_electrolyzer(request.electrolyzer_id)?;
    let power_grid = power_grid_fetcher.get_power_grid()?;
    let current_simulation_id = simulation_selection_client.expect_current_selection(user.id())?;
    let mut next_simulation = SimulationState::default();
    next_simulation.electrolyzer_id = electrolyzer.id;
    let next_simulation = simulation_client.create_simulation_state(&next_simulation)?;
    let next_url = &format!("simulation/{}", next_simulation.id);
    let location = client_context.mut_location();
    location.set_path(&next_url);
    simulation_selection_client.select(user.id().clone(), next_simulation.id)?;

    Component::component(
        HtmxHeadersBuilder::new()
            .replace_url(&location.build_url())
            .build(),
        ExecuteSimulationResponse {
            simulation_view: SimulationViewBuilder::new()
                .generation_range(DateTimeRange {
                    start: String::from("2023-01-01T00:00"),
                    end: String::from("2023-07-31T23:59"),
                })
                .electrolyzer_selector(ElectrolyzerSelectorTemplate {
                    electrolyzers: electrolyzer_client.list_electrolyzers()?,
                    selected_id: electrolyzer.id,
                    select_electrolyzer_listener: EventListenerBuilder::new()
                        .event(ClientEvent::SelectElectrolyzer)
                        .endpoint("/electrolyzer_selector")
                        .target("#electrolyzer-selector")
                        .swap(HtmxSwap::OuterHtml)
                        .build(),
                })
                .build(),
            simulation_result: simulate(
                current_simulation_id,
                &power_grid,
                &electrolyzer,
                &request.simulation_time_range,
                simulation_client.inner(),
            )?,
        },
    )
}
