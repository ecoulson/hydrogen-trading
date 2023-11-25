use askama::Template;
use rocket::{post, State};

use crate::{
    client::events::ClientEvent,
    components::{
        component::{Component, ComponentResponse},
        event::{EventListener, EventListenerBuilder},
    },
    logic::simulation::SimulationState,
    persistance::{simulation::SimulationClient, simulation_selection::SimulationSelectionClient},
    responders::{client_context::ClientContext, htmx_responder::HtmxHeadersBuilder},
    schema::{errors::BannerError, user::User},
};

#[derive(Debug, Template)]
#[template(path = "components/list_simulations.html")]
pub struct ListSimulationResponse {
    pub simulations: Vec<SimulationState>,
    pub create_electrolyzer_listener: EventListener,
}
#[post("/list_simulations")]
pub fn list_simulation_handler(
    user: User,
    client_context: ClientContext,
    simulation_client: &State<Box<dyn SimulationClient>>,
    simulation_selection_client: &State<Box<dyn SimulationSelectionClient>>,
) -> ComponentResponse<ListSimulationResponse, BannerError> {
    let mut client_context = client_context;
    let location = client_context.mut_location();
    location.set_path("");
    simulation_selection_client.unselect(user.id())?;

    Component::component(
        HtmxHeadersBuilder::new()
            .trigger(ClientEvent::ListSimulations)
            .replace_url(&location.build_url())
            .build(),
        ListSimulationResponse {
            simulations: simulation_client.list_simulations()?,
            create_electrolyzer_listener: EventListenerBuilder::new()
                .event(ClientEvent::CreateElectrolyzer)
                .endpoint("initialize_simulation")
                .target("#dataplane")
                .build(),
        },
    )
}
