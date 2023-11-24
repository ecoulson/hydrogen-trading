use rocket::{form::Form, post, FromForm, State};

use crate::{
    components::component::{Component, ComponentResponse},
    persistance::{
        electrolyzer::ElectrolyzerClient, simulation::SimulationClient,
        simulation_selection::SimulationSelectionClient,
    },
    responders::{client_context::ClientContext, htmx_responder::HtmxHeadersBuilder},
    schema::{
        errors::BannerError, simulation_schema::SimulationId, time::DateTimeRange, user::User,
    },
    templates::{
        list_electrolyzers_template::ElectrolyzerSelectorTemplate, simulation_view::SimulationView,
    },
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
    simulation_selection.select(user.id().clone(), request.simulation_id)?;
    location.set_path(&next_url);

    Component::component(
        HtmxHeadersBuilder::new()
            .replace_url(&location.build_url())
            .trigger("simulation-selected")
            .build(),
        SimulationView {
            generation_range: DateTimeRange {
                start: String::from("2023-01-01T00:00"),
                end: String::from("2023-07-31T23:59"),
            },
            electrolyzer_selector: ElectrolyzerSelectorTemplate {
                electrolyzers,
                selected_id: simulation.electrolyzer_id,
            },
        },
    )
}
