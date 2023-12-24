use rocket::{form::Form, post, FromForm, State};

use crate::{
    client::events::ClientEvent,
    components::{
        component::{Component, ComponentResponse},
        electrolyzer::ElectrolyzerSelector,
        error::BannerError,
        simulation::SimulationView,
    },
    persistance::{
        electrolyzer::ElectrolyzerClient, simulation::SimulationClient,
        simulation_selection::SimulationSelectionClient,
    },
    responders::{client_context::ClientContext, htmx_responder::HtmxHeadersBuilder},
    schema::{simulation::SimulationId, time::DateTimeRange, user::User},
};

#[derive(Debug, FromForm)]
pub struct SelectSimulationRequest {
    pub simulation_id: SimulationId,
}

#[post("/select_simulation", data = "<request>")]
pub fn select_simulation_handler(
    request: Form<SelectSimulationRequest>,
    user: User,
    client_context: ClientContext,
    simulation_client: &State<Box<dyn SimulationClient>>,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
    simulation_selection: &State<Box<dyn SimulationSelectionClient>>,
) -> ComponentResponse<SimulationView, BannerError> {
    let mut client_context = client_context;
    let simulation = simulation_client.get_simulation_state(&request.simulation_id)?;
    let electrolyzers = electrolyzer_client.list_electrolyzers()?;
    let next_url = &format!("simulation/{}", simulation.id);
    let location = client_context.mut_location();
    simulation_selection.select(user.id, request.simulation_id)?;
    location.set_path(&next_url);

    Component::component(
        HtmxHeadersBuilder::new()
            .replace_url(&location.build_url())
            .trigger(ClientEvent::SelectSimulation)
            .build(),
        SimulationView::render(
            DateTimeRange {
                start: String::from("2023-01-01T00:00"),
                end: String::from("2023-07-31T23:59"),
            },
            ElectrolyzerSelector::render(simulation.electrolyzer_id, electrolyzers),
        ),
    )
}
