use rocket::{post, State};

use crate::{
    client::events::ClientEvent,
    components::{
        component::{Component, ComponentResponse},
        error::BannerError,
        simulation::SimulationList,
    },
    persistance::{simulation::SimulationClient, simulation_selection::SimulationSelectionClient},
    responders::{client_context::ClientContext, htmx_responder::HtmxHeadersBuilder},
    schema::user::User,
};

#[post("/list_simulations")]
pub fn list_simulation_handler(
    user: User,
    client_context: ClientContext,
    simulation_client: &State<Box<dyn SimulationClient>>,
    simulation_selection_client: &State<Box<dyn SimulationSelectionClient>>,
) -> ComponentResponse<SimulationList, BannerError> {
    let mut client_context = client_context;
    let location = client_context.mut_location();
    location.set_path("");
    simulation_selection_client.unselect(&user.id)?;

    Component::component(
        HtmxHeadersBuilder::new()
            .trigger(ClientEvent::ListSimulations)
            .replace_url(&location.build_url())
            .build(),
        SimulationList::render(simulation_client.list_simulations()?),
    )
}
