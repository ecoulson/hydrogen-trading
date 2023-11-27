use rocket::{form::Form, post, FromForm, State};

use crate::{
    components::{
        button::ButtonBuilder,
        component::{Component, ComponentResponse},
    },
    persistance::{
        electrolyzer::ElectrolyzerClient, simulation::SimulationClient,
        simulation_selection::SimulationSelectionClient,
    },
    schema::{
        electrolyzer::{ElectrolyzerDetails, ElectrolyzerDetailsBuilder, ElectrolyzerId},
        errors::BannerError,
        user::User,
    },
};

#[derive(FromForm)]
pub struct GetElectrolyzerRequest {
    pub electrolyzer_id: ElectrolyzerId,
}

#[post("/get_electrolyzer", data = "<request>")]
pub fn get_electrolyzer_handler(
    request: Form<GetElectrolyzerRequest>,
    user: User,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
    simulation_client: &State<Box<dyn SimulationClient>>,
    simulation_selection_client: &State<Box<dyn SimulationSelectionClient>>,
) -> ComponentResponse<ElectrolyzerDetails, BannerError> {
    let electrolyzer = electrolyzer_client.get_electrolyzer(request.electrolyzer_id)?;
    let simulation_id = simulation_selection_client.current_selection(user.id())?;

    if simulation_id.is_none() {
        return Component::basic(
            ElectrolyzerDetailsBuilder::new()
                .electrolyzer(electrolyzer)
                .select_electrolyzer_button(
                    ButtonBuilder::new()
                        .endpoint("/select_electrolyzer")
                        .target("#sidebar")
                        .disabled()
                        .text("Use")
                        .build(),
                )
                .build(),
        );
    }

    let simulation = simulation_client.get_simulation_state(&simulation_id.unwrap())?;

    if request.electrolyzer_id == simulation.electrolyzer_id {
        return Component::basic(
            ElectrolyzerDetailsBuilder::new()
                .electrolyzer(electrolyzer)
                .selected()
                .build(),
        );
    }

    Component::basic(
        ElectrolyzerDetailsBuilder::new()
            .electrolyzer(electrolyzer)
            .build(),
    )
}
